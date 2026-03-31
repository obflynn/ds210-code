use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row}; // 
use crate::query::{Aggregation, Condition, Query};


// Helper function to check if a row satisfies a given condition
fn row_satisfies_condition(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        //check if the value in the column matches the target value
        Condition::Equal(col_name, value) => {
            let index = dataset.column_index(col_name); //find column index
            row.get_value(index) == value               //compare value in row
        }

        //row satisfies condition if the inner condition is not satisfied
        Condition::Not(cond) => !row_satisfies_condition(row, dataset, cond),

        //row satisfies condition if both inner conditions are satisfied
        Condition::And(cond1, cond2) => {
            row_satisfies_condition(row, dataset, cond1)
                && row_satisfies_condition(row, dataset, cond2)
        }

        //row satisfies condition if at least one condition is satisfied
        Condition::Or(cond1, cond2) => {
            row_satisfies_condition(row, dataset, cond1)
                || row_satisfies_condition(row, dataset, cond2)
        }
    }
}

// Filters a dataset based on the given condition and returns a new dataset
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered = Dataset::new(dataset.columns().clone());

    for row in dataset.iter() {
        if row_satisfies_condition(row, dataset, filter) {
            filtered.add_row(row.clone()); // clone needed because of ownership
        }
    }

    filtered
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut grouped_datasets: HashMap<Value, Dataset> = HashMap::new(); // new dataset to store group_by_column subset of original dataset
    let column_i = dataset.column_index(group_by_column); // index of the group_by_column in the dataset columns

    for row in dataset.iter() { // iterate over the rows of dataset
        let key: Value = row.get_value(column_i).clone(); // key is the value of the group_by_column in the row
        
        grouped_datasets // checks that the group_by_column value in the row is already a key or adds it as a key w/in a new subset/dataset in the HashMap
            .entry(key.clone()) // use value of group_by_column in the row as the HashMap key
            .or_insert_with(|| Dataset::new(dataset.columns().clone())); // if the key isn't already w/in the HashMap add it with new subset/dataset
        grouped_datasets
            .get_mut(&key) // mutable ref to the original dataset that corresponds to the key
            .unwrap() // unwrap required because get_mut returns an Option<&mut Dataset>, but shouldn't cause an error due to the use of .entry()/.or_insert_with() above
            .add_row(row.clone()); // copy of the row ref-ed above is added to the subset/dataset
    }
    grouped_datasets
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>,aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut results = HashMap::new();
    for (group_key, group_dataset) in dataset {

        // figure out what aggregation result to calculate for this group
        let agg_value = match aggregation {

            // count just return the number of rows in this group
            Aggregation::Count(_) => {
                Value::Integer(group_dataset.len() as i32)
            }

            //sum add up all integer values in the specified column
            Aggregation::Sum(col_name) => {
                let col_index = group_dataset.column_index(col_name);
                let mut sum = 0;

                // loop over each row and add the value if it's an integer
                for row in group_dataset.iter() {
                    if let Value::Integer(n) = row.get_value(col_index) {
                        sum += *n;
                    }
                }
                Value::Integer(sum)
            }

            // average, sum the integers then divide by the number of rows
            Aggregation::Average(col_name) => {
                let col_index = group_dataset.column_index(col_name);
                let mut sum = 0;

                for row in group_dataset.iter() {
                    if let Value::Integer(n) = row.get_value(col_index) {
                        sum += *n;
                    }
                }

                let count = group_dataset.len();
                if count > 0 {
                    Value::Integer(sum / count as i32)
                } else {
                    Value::Integer(0)
                }
            }
        };

        // store result in results map
        results.insert(group_key, agg_value);
    }
    results
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}
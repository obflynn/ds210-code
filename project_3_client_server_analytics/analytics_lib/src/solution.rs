use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
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
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut results: HashMap<Value, Value> = HashMap::new();
    // Iterate over each group
    for (group_key, group_dataset) in dataset {
        let agg_value = match aggregation {
            Aggregation::Count(_) => {
                // Count (the number of rows in this group)
                Value::Integer(group_dataset.len() as i32)
            }

            Aggregation::Sum(col_name) | Aggregation::Average(col_name) => {
                let col_index = group_dataset.column_index(col_name);

                // set sum to 0
                let mut sum: i32 = 0;

                //loop over each row in the group
                for row in group_dataset.iter() {
                    match row.get_value(col_index) {
                        Value::Integer(n) => sum += *n, //add integer values
                        _ => (), //ignore non-integer values
                    }
                }

                // Determine final aggregated value based on aggregation type
                match aggregation {
                    Aggregation::Sum(_) => Value::Integer(sum),
                    Aggregation::Average(_) => {
                        let count = group_dataset.len();
                        // Avoid division by zero
                        if count > 0 {
                            Value::Integer(sum / count as i32)
                        } else {
                            Value::Integer(0)
                        }
                    }
                    _ => panic!("Unexpected aggregation type"),
                }
            }
        };
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
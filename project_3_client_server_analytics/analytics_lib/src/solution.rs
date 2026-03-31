use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row}; // 
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered = Dataset::new(dataset.columns().clone());

    for row in dataset.iter() {
        match condition {
            Condition::Equal(column_name, value) => {
                let col_index = dataset.column_index(&column_name);
                if row.get_value(col_index) == &value {
                    filtered.add_row(row.clone());
                }
            }
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


pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
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
use analytics_lib::{dataset::Dataset, query::Query};

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    return input_dataset.clone();
}

pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    println!("fast_rpc called");
    // input_dataset is cloned since it is passed by reference and we want to return a new dataset to the client that is the result of executing the query on the input dataset
    let final_dataset = query(input_dataset.clone()); 
    return final_dataset;

    // mirrored solution from client/src/solution.rs commented out since I don't think this is how you're supposed to implement this
    // let resulting_dataset = analytics_lib::solution::compute_query_on_dataset(input_dataset, query); 
    // return resulting_dataset; 
}
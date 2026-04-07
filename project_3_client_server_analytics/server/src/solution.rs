use analytics_lib::{dataset::Dataset, query::Query};
use analytics_lib::solution::compute_query_on_dataset;
pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    input_dataset.clone() // input_dataset is cloned and no query is performed on server side (occurs on client side making it slow)
}

pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    println!("fast_rpc called");
    compute_query_on_dataset(input_dataset, &query) // function implements query logic on the input_dataset on the server side
}

use analytics_lib::{dataset::Dataset, query::Query};

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    input_dataset.clone()
}

pub fn fast_rpc(input_dataset: &Dataset, _query: Query) -> Dataset {
    println!("fast_rpc called");
    // For now, just return a clone of the input dataset (same as slow_rpc)
    input_dataset.clone()
}

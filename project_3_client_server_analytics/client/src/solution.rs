use analytics_lib::{dataset::Dataset, query::Query, solution::compute_query_on_dataset};
use interface::RPCInterfaceClient;
use tarpc::context::Context;

pub async fn run_hello(rpc_client: &RPCInterfaceClient) {
    let result = rpc_client.hello(Context::current()).await.unwrap();
    println!("The server says: `{}`", result);
}

pub async fn run_slow_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using slow_rpc");
    let dataset = rpc_client.slow_rpc(Context::current()).await.unwrap(); // mirrors let result code from run_hello
    let result = compute_query_on_dataset(&dataset, &query); // reference dataset to avoid moving it, query computes result on the dataset and returns it to client as new dataset
    println!("slow_rpc result: {:?}", result); // print result in terminal to verify it is correct
    return result; 
}

pub async fn run_fast_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using fast_rpc");

    // You should call fast_rpc here and not slow_rpc.
    todo!("Implement this");
}

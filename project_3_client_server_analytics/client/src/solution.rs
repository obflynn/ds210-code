use analytics_lib::{dataset::Dataset, query::Query, solution::compute_query_on_dataset};
use interface::RPCInterfaceClient;
use tarpc::context::Context;

pub async fn run_hello(rpc_client: &RPCInterfaceClient) {
    let result = rpc_client.hello(Context::current()).await.unwrap();
    println!("The server says: `{}`", result);

}pub async fn run_slow_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using slow_rpc");

    //get dataset from the server
    let dataset = rpc_client.slow_rpc(Context::current()).await.unwrap();

    //compute the query (pass references)
    let result_dataset = compute_query_on_dataset(&dataset, &query);

    result_dataset
}
pub async fn run_fast_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using fast_rpc");

    // Call server's fast_rpc with the current tarpc Context
    let result_dataset = rpc_client
        .fast_rpc(Context::current(), query)
        .await // wait for the server to process the query and return the result
        .unwrap(); // unwraps result and panics if there was an error in the query execution

    // Already a Dataset, no further unwrap needed
    result_dataset
}
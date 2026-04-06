extern crate tarpc;
use analytics_lib::dataset::Value;
use std::time::Instant;
use std::io::BufRead;
use analytics_lib::query::{Query, Aggregation, Condition};
use client::{start_client, solution};
fn parse_query_from_string(input: String) -> Query {
    let input = input.trim();
    //check if query is for albums or grades by looking for band,
    //then build a query using columns for that dataset
    let (filter, group_by, aggregate) = if input.contains("band") {
        (
            Condition::Equal("band".to_string(), Value::String("Meshuggah".to_string())),
            "album".to_string(),
            Aggregation::Average("rating".to_string())
        )
    } else {
        (
            Condition::Equal("section".to_string(), Value::String("A1".to_string())),
            "grade".to_string(),
            Aggregation::Count("name".to_string())
        )
    };

    Query::new(filter, group_by, aggregate)
}
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}

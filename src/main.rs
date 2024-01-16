// main.rs
mod cli;
use cli::parser::{parse_and_execute};

#[tokio::main] // This enables the Tokio runtime
async fn main() {
    parse_and_execute().await; // Call your async function with .await
}

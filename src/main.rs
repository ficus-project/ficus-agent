use futures::executor::block_on;

use crate::analyzer::analyze_resources;

mod analyzer;
mod connectors;


#[tokio::main]
async fn main() {
  block_on(analyze_resources());
}

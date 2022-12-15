use analyzers::analyzer::analyze_resources;
use futures::executor::block_on;

mod analyzers;
mod connectors;


#[tokio::main]
async fn main() {
  block_on(analyze_resources());
}

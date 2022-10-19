use std::collections::HashMap;

use async_trait::async_trait;

use crate::models::{resources::{VirtualMachine, UsageMetric}, errors::FetchResourceError};


#[async_trait]
pub trait VirtualMachineDatasource {
  async fn new() -> Self;

  async fn list_virtual_machines(&self) -> Result<Vec<VirtualMachine>, FetchResourceError> {
    Err(FetchResourceError { message: String::from("Not implemented") })
  }

  async fn measure_virtual_machines_usage(&self, _identifiers: Vec<&String>) -> Result<HashMap<String, UsageMetric>, FetchResourceError> {
    Err(FetchResourceError { message: String::from("Not implemented") })
  }
}

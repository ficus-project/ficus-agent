use std::{collections::HashMap};

use async_trait::async_trait;

use crate::models::{resources::{VirtualMachine, UsageMetric}, errors::FetchResourceError};


#[async_trait(?Send)]
pub trait VirtualMachineProvider {
  async fn list_virtual_machines(&self) -> Result<Box<Vec<VirtualMachine>>, FetchResourceError> {
    Err(FetchResourceError { message: String::from("Not implemented") })
  }

  async fn measure_virtual_machines_usage(&self, _identifiers: &Vec<String>, _from_timestamp: u64, _to_timestamp: u64) -> Result<Box<HashMap<String, UsageMetric>>, FetchResourceError> {
    Err(FetchResourceError { message: String::from("Not implemented") })
  }
}

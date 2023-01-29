use std::{collections::HashMap};

use async_trait::async_trait;

use crate::models::{resources::{CloudFunction, UsageMetric}, errors::FetchResourceError};


#[async_trait(?Send)]
pub trait CloudFunctionProvider {
  async fn list_cloud_functions(&self) -> Result<Box<Vec<CloudFunction>>, FetchResourceError> {
    Err(FetchResourceError { message: String::from("Not implemented") })
  }

  async fn measure_cloud_functions_usage(&self, _function_names: Vec<String>, _from_timestamp: u64, _to_timestamp: u64) -> Result<Box<HashMap<String, HashMap<String, UsageMetric>>>, FetchResourceError> {
    Err(FetchResourceError { message: String::from("Not implemented") })
  }
}

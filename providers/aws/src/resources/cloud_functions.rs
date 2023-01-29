use std::collections::HashMap;

use crate::PROVIDER_IDENTIFIER;
use crate::connectors::cloudwatch::get_lambda_insight_usage;
use crate::{utils::load_config, connectors::lambda::get_lambdas};
use async_trait::async_trait;
use aws_sdk_cloudwatch::{Client as CloudWatchClient, types::DateTime};
use aws_sdk_lambda::{Client as LambdaClient,model::FunctionConfiguration};
use ficus_agent_lib::models::resources::{CloudFunction, UsageMetric};
use ficus_agent_lib::models::errors::FetchResourceError;
use ficus_agent_lib::resources::cloud_functions::CloudFunctionProvider;

pub struct AwsCloudFunctionProvider {
  lambda_client: LambdaClient,
  cloudwatch_client: CloudWatchClient,
}

impl AwsCloudFunctionProvider {
  pub async fn new() -> Self {
    let config = load_config().await;
    let lambda_client = LambdaClient::new(&config);
    let cloudwatch_client = CloudWatchClient::new(&config);

    AwsCloudFunctionProvider {
      lambda_client,
      cloudwatch_client,
    }
  }
}


#[async_trait(?Send)]
impl CloudFunctionProvider for AwsCloudFunctionProvider {
  async fn list_cloud_functions(&self) -> Result<Box<Vec<CloudFunction>>, FetchResourceError> {
    let mut functions: Vec<CloudFunction> = vec![];
    let mut next_marker: Option<String> = None;
    
    loop {
      match get_lambdas(&self.lambda_client, next_marker).await {
        Ok((functions_configurations, marker)) => {
          for lambda in functions_configurations {
            let function = map_lambda_to_model(lambda);

            functions.push(function);
          }
          next_marker = marker;
        },
        Err(error) => {
          println!("Failed to get lambdas: {}", error);
          return Err(FetchResourceError { message: error.to_string() });
        }
      };
      if next_marker.is_none() { break; }
    }

    Ok(Box::new(functions))
  }

  async fn measure_cloud_functions_usage(&self, function_names: Vec<String>, from_timestamp: u64, to_timestamp: u64) -> Result<Box<HashMap<String, HashMap<String, UsageMetric>>>, FetchResourceError> {
    let mut metrics: HashMap<String, HashMap<String, UsageMetric>> = HashMap::new();
    
    for function_name in function_names {
      let mut function_metrics = HashMap::new();
      for metric_name in [String::from("total_memory"), String::from("memory_utilization"), String::from("total_network")] {
        match get_lambda_insight_usage(&self.cloudwatch_client, &metric_name, &function_name, DateTime::from_secs(from_timestamp as i64), DateTime::from_secs(to_timestamp as i64), 3600).await {
          Ok(datapoints) => {
            for datapoint in datapoints {
              let timestamp = match datapoint.timestamp() { Some(datapoint_timestamp) => { Some(datapoint_timestamp.secs()) }, None => None };
              
              function_metrics.insert(metric_name.clone(), UsageMetric { timestamp, average: datapoint.average(), sum: datapoint.sum() });
            }
          },
          Err(error) => {
            println!("Failed to get Lambda cloudwatch metrics: {}", error);
            return Err(FetchResourceError { message: error.to_string() });
          }
        };
      }
      metrics.insert(function_name.clone(), function_metrics);
    }
    
    println!("{:?}", metrics);
    Ok(Box::new(metrics))
  }
}

fn map_lambda_to_model(lambda: FunctionConfiguration) -> CloudFunction {
  let name = lambda.function_name().map(|n| String::from(n));
  let description = lambda.description().map(|d| String::from(d));
  let code_size = lambda.code_size();
  let memory = lambda.memory_size();

  CloudFunction { name, description, memory, code_size: Some(code_size), provider: String::from(PROVIDER_IDENTIFIER) }
}

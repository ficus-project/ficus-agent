use std::collections::HashMap;
use async_trait::async_trait;
use aws_sdk_ec2::{Client as Ec2Client};
use aws_sdk_cloudwatch::{Client as CloudWatchClient, types::DateTime};
use ficus_agent_lib::models::{errors::FetchResourceError, resources::UsageMetric};
use ficus_agent_lib::models::resources::VirtualMachine;
use ficus_agent_lib::resources::virtual_machines::VirtualMachineDatasource;

use crate::{utils::load_config, connectors::{ec2::{get_ec2, get_instance_types}, cloudwatch::get_ec2_cpu_usage}};

pub struct AwsVirtualMachineDatasource {
  ec2_client: Ec2Client,
  cloudwatch_client: CloudWatchClient,
  instance_types: HashMap<String, Ec2InstanceType>,
}

struct Ec2InstanceType {
  _name: String,
  memory: i64,
}


#[async_trait]
impl VirtualMachineDatasource for AwsVirtualMachineDatasource {

  async fn new() -> Self {
    let config = load_config().await;
    let ec2_client = Ec2Client::new(&config);
    let cloudwatch_client = CloudWatchClient::new(&config);

    let instance_types = get_all_instance_types(&ec2_client).await.unwrap_or(HashMap::new());

    AwsVirtualMachineDatasource {
      instance_types,
      ec2_client,
      cloudwatch_client
    }
  }

  async fn list_virtual_machines(&self) -> Result<Vec<VirtualMachine>, FetchResourceError> {
    let mut instances: Vec<VirtualMachine> = vec![];
    let mut next_token: Option<String> = None;

    loop {
      match get_ec2(&self.ec2_client, next_token).await {
        Ok((ec2_instances, token)) => {
          for ec2_instance in ec2_instances {
            let identifier = match ec2_instance.instance_id() { Some(ec2_identifier) => { Some(String::from(ec2_identifier)) }, None => None };
            let cpu_core = match ec2_instance.cpu_options() { Some(cpu_options) => { cpu_options.core_count() }, None => None };
            let ec2_instance_type = match ec2_instance.instance_type() { Some(instance_type) => { self.instance_types.get(instance_type.as_str()) }, None => None };
            let memory_in_mb = match ec2_instance_type { Some(instance_type) => { Some(instance_type.memory) }, None => None };
            
            instances.push(VirtualMachine { identifier, cpu_core, memory_in_mb })
          }
          next_token = token;
        },
        Err(error) => {
          println!("Failed to get EC2: {}", error);
          return Err(FetchResourceError { message: error.to_string() });
        }
      };
      if next_token.is_none() { break; }
    }
    Ok(instances)
  }

  async fn measure_virtual_machines_usage(&self, identifiers: Vec<&String>) -> Result<HashMap<String, UsageMetric>, FetchResourceError> {
    let mut metrics: HashMap<String, UsageMetric> = HashMap::new();

    for identifier in identifiers {
      match get_ec2_cpu_usage(&self.cloudwatch_client, &identifier, DateTime::from_secs(	1668862912), DateTime::from_secs(	1668898912), 3600).await {
        Ok(datapoints) => {
          for datapoint in datapoints {
            let timestamp = match datapoint.timestamp() { Some(datapoint_timestamp) => { Some(datapoint_timestamp.secs()) }, None => None };
            
            metrics.insert(identifier.clone(), UsageMetric { timestamp, average: datapoint.average(), sum: datapoint.sum() });
          }
        },
        Err(error) => {
          println!("Failed to get EC2 cloudwatch metrics: {}", error);
          return Err(FetchResourceError { message: error.to_string() });
        }
      };
    }
    Ok(metrics)
  }
}

async fn get_all_instance_types(client: &Ec2Client) -> Result<HashMap<String, Ec2InstanceType>, FetchResourceError> {
  let mut instance_types: HashMap<String, Ec2InstanceType> = HashMap::new();
  let mut next_token: Option<String> = None;

  loop {
    match get_instance_types(&client, next_token).await {
      Ok((ec2_instance_types, token)) => {
        for ec2_instance_type in ec2_instance_types {
          let type_name = String::from(ec2_instance_type.instance_type().unwrap().as_str());
          let memory = ec2_instance_type.memory_info().unwrap().size_in_mi_b().unwrap();
          instance_types.insert(type_name.clone(), Ec2InstanceType {
            _name: type_name,
            memory,
          });
        }
        next_token = token;
      },
      Err(error) => {
        println!("Failed to get EC2 instance types: {}", error);
        return Err(FetchResourceError { message: error.to_string() });
      }
    };
    if next_token.is_none() { break; }
  }

  Ok(instance_types)
}

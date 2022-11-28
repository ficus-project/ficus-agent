use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use ficus_agent_aws::resources::virtual_machines::AwsVirtualMachineProvider;
use ficus_agent_lib::{resources::virtual_machines::VirtualMachineProvider, models::resources::{VirtualMachine, UsageMetric}};
use ficus_agent_mock::resources::virtual_machines::MockVirtualMachineProvider;

use crate::connectors::influxdb::InfluxdbWriter;


const DAY_DURATION_SEC: u64 = 24 * 60 * 60;

pub async fn analyze_resources() {
  let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
  let past_day_timestamp = now_timestamp - DAY_DURATION_SEC;

  analyze_virtual_machines(past_day_timestamp, now_timestamp).await;
}

async fn analyze_virtual_machines(from_timestamp: u64, to_timestamp: u64) {
  // Init available providers
  let providers: Vec<Box<dyn VirtualMachineProvider>> = vec![
    #[cfg(feature = "aws")]
    Box::new(AwsVirtualMachineProvider::new().await),
    #[cfg(feature = "mock")]
    Box::new(MockVirtualMachineProvider::new().await),
  ];

  // Fetching resources & consumption
  let mut virtual_machines: Vec<VirtualMachine> = vec![];
  let mut vms_usage: HashMap<String, UsageMetric> = HashMap::new();
  for provider in providers {
    let vms_result = provider.list_virtual_machines().await;
    if let Ok(mut vms) = vms_result {
      let vms_identifiers: Vec<String> = vms.iter()
        .flat_map(|vm| vm.identifier.clone()).collect::<Vec<String>>();
      if let Ok(metrics) = provider.measure_virtual_machines_usage(&vms_identifiers, from_timestamp, to_timestamp).await {
        vms_usage.extend(*metrics);
      }

      virtual_machines.append((*vms).as_mut());
    }
  }
  
  // Write to influxdb
  let mut metric_sent_count: i32 = 0;
  let influxdb_writer = InfluxdbWriter::new();
  for (vm, usage) in vms_usage {
    let mut tags = HashMap::new();
    tags.insert(String::from("host"), vm);
    let mut fields = HashMap::new();
    if let Some(average) = usage.average {
      fields.insert(String::from("average"), average);
    }
    if let Some(sum) = usage.sum {
      fields.insert(String::from("sum"), sum);
    }
    if let Some(timestamp) = usage.timestamp {
      influxdb_writer.send_metric(String::from("cpu"), timestamp, &tags, &fields).await.unwrap();
      metric_sent_count = metric_sent_count + 1;
    }
  }
  println!("Sent {} metrics to influx, for {} virtual machines", metric_sent_count, virtual_machines.len());
}

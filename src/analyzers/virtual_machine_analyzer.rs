use std::{collections::HashMap, time::{UNIX_EPOCH, SystemTime}};

use ficus_agent_aws::resources::virtual_machines::AwsVirtualMachineProvider;
use ficus_agent_lib::{models::resources::{UsageMetric, VirtualMachine}, resources::virtual_machines::VirtualMachineProvider};
use ficus_agent_mock::resources::virtual_machines::MockVirtualMachineProvider;

use crate::connectors::influxdb::InfluxdbWriter;



pub async fn analyze_virtual_machines(from_timestamp: u64, to_timestamp: u64) {
  // Fetching resources & consumption
  let (vms, vms_usage) = fetch_vm_and_consumption(from_timestamp, to_timestamp).await;

  // Storing data
  store_vms_existence_data(vms).await;
  store_vms_usage_data(vms_usage).await;
}

async fn fetch_vm_and_consumption(from_timestamp: u64, to_timestamp: u64) -> (Vec<VirtualMachine>, HashMap<String, UsageMetric>) {
  let providers = get_vm_providers().await;
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

  (virtual_machines, vms_usage)
}

async fn get_vm_providers() -> Vec<Box<dyn VirtualMachineProvider>> {
  vec![
    #[cfg(feature = "aws")]
    Box::new(AwsVirtualMachineProvider::new().await),
    #[cfg(feature = "mock")]
    Box::new(MockVirtualMachineProvider::new().await),
  ]
}

async fn store_vms_existence_data(vms: Vec<VirtualMachine>) {
  let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
  let influxdb_writer = InfluxdbWriter::new();
  println!("Sending {} virtual machines data to influx", vms.len());
  for vm in vms {
    let mut bool_fields: HashMap<String, bool> = HashMap::new();
    if let Some(is_running) = vm.is_running { bool_fields.insert(String::from("is_running"), is_running); }

    let (tags, fields) = build_tags_and_fields_for_vm(vm).await;
    
    influxdb_writer.send_metric(String::from("vm"), now_timestamp, &tags, &fields).await.unwrap();
    influxdb_writer.send_metric(String::from("vm"), now_timestamp, &tags, &bool_fields).await.unwrap();
  }
}

async fn build_tags_and_fields_for_vm(vm: VirtualMachine) -> (HashMap<String, String>, HashMap<String, i64>) {
  let mut tags = HashMap::new();
  if let Some(vm_identifier) = vm.identifier { tags.insert(String::from("id"), vm_identifier); }
  for (tag_key, tag_value) in vm.tags {
    tags.insert("tag:".to_owned() + &tag_key, tag_value.clone());
    if tag_key == "Name" { tags.insert(String::from("name"), tag_value); }
  }

  let mut int_fields: HashMap<String, i64> = HashMap::new();
  if let Some(cpu_cores) = vm.cpu_cores { int_fields.insert(String::from("cpu_cores"), cpu_cores as i64); }
  if let Some(cpu_threads) = vm.cpu_threads { int_fields.insert(String::from("cpu_threads"), cpu_threads as i64); }
  if let Some(memory_in_mb) = vm.memory_in_mb { int_fields.insert(String::from("memory_in_mb"), memory_in_mb); }
  let mut bool_fields: HashMap<String, bool> = HashMap::new();
  if let Some(is_running) = vm.is_running { bool_fields.insert(String::from("is_running"), is_running); }
  
  (tags, int_fields)
}

async fn store_vms_usage_data(vms_usage: HashMap<String, UsageMetric>) {
  let mut metric_sent_count: i32 = 0;
  let vms_count = vms_usage.len();
  let influxdb_writer = InfluxdbWriter::new();
  for (vm, usage) in vms_usage {
    let mut tags = HashMap::new();
    tags.insert(String::from("host"), vm);

    let mut fields = HashMap::new();
    if let Some(average) = usage.average { fields.insert(String::from("average"), average); }
    if let Some(sum) = usage.sum { fields.insert(String::from("sum"), sum); }
    if let Some(timestamp) = usage.timestamp {
      influxdb_writer.send_metric(String::from("vm_cpu"), timestamp, &tags, &fields).await.unwrap();
      metric_sent_count = metric_sent_count + 1;
    }
  }
  println!("Sent {} usage metrics to influx, related to the consumption of {} virtual machines", metric_sent_count, vms_count);
}

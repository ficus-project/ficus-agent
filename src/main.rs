
use std::collections::HashMap;

use ficus_agent_lib::{resources::virtual_machines::VirtualMachineDatasource, models::resources::UsageMetric};
#[cfg(feature = "aws")]
use ficus_agent_aws::resources::virtual_machines::AwsVirtualMachineDatasource;
#[cfg(feature = "mock")]
use ficus_agent_mock::resources::virtual_machines::MockVirtualMachineDatasource;
use futures::executor::block_on;

mod aggregator;


#[tokio::main]
async fn main() {
  let mock_virtual_machine_datasource = block_on(MockVirtualMachineDatasource::new());
  let mock_vms = block_on(mock_virtual_machine_datasource.list_virtual_machines());
  println!("Mock data: {:?}", mock_vms);

  let aws_virtual_machine_datasource = block_on(AwsVirtualMachineDatasource::new());
  let vms_result = block_on(aws_virtual_machine_datasource.list_virtual_machines());
  
  let mut vms_usage: HashMap<String, UsageMetric> = HashMap::new();
  if let Ok(vms) = &vms_result {
    let vms_identifiers: Vec<&String> = vms.into_iter()
      .flat_map(|vm| &vm.identifier).collect();
    println!("vm list: {:?}", vms_identifiers);
    if let Ok(metrics) = block_on(aws_virtual_machine_datasource.measure_virtual_machines_usage(vms_identifiers)) {
      vms_usage.extend(metrics);
    }
  }
  
  println!("AWS vms: {:?}", vms_result);
  println!("AWS metrics: {:?}", vms_usage);
}

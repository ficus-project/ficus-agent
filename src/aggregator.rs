use std::collections::HashMap;

use ficus_agent_lib::{resources::virtual_machines::VirtualMachineDatasource, models::resources::UsageMetric};



// pub async fn fetch_virtual_machines_data<T: VirtualMachineDatasource>(datasource: &T) {
//   let vms_result = datasource.list_virtual_machines().await;
  
//   let mut vms_usage: HashMap<String, UsageMetric> = HashMap::new();
//   if let Ok(vms) = &vms_result {
//     let vms_identifiers: Vec<&String> = vms.into_iter()
//       .flat_map(|vm| &vm.identifier).collect();
//     println!("vm list: {:?}", vms_identifiers);
//     if let Ok(metrics) = datasource.measure_virtual_machines_usage(vms_identifiers).await {
//       vms_usage.extend(metrics);
//     }
//   }
// }

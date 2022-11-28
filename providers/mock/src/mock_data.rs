use ficus_agent_lib::models::resources::VirtualMachine;

pub struct MockData {}

impl MockData {
  pub fn generate_virtual_machines() -> Vec<VirtualMachine> {
    return vec![
      VirtualMachine {
        identifier: Some(String::from("corvo")),
        cpu_core: Some(4),
        memory_in_mb: Some(4048),
      },
      VirtualMachine {
        identifier: Some(String::from("orero")),
        cpu_core: Some(8),
        memory_in_mb: Some(2048),
      }
    ]
  }
}

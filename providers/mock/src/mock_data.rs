use std::collections::HashMap;

use ficus_agent_lib::models::resources::VirtualMachine;

use crate::PROVIDER_IDENTIFIER;

pub struct MockData {}

impl MockData {
  pub fn generate_virtual_machines() -> Vec<VirtualMachine> {
    return vec![
      VirtualMachine {
        identifier: Some(String::from("corvo")),
        cpu_cores: Some(4),
        cpu_threads: Some(8),
        memory_in_mb: Some(4048),
        is_running: Some(true),
        tags: HashMap::new(),
        provider: String::from(PROVIDER_IDENTIFIER),
      },
      VirtualMachine {
        identifier: Some(String::from("orero")),
        cpu_cores: Some(8),
        cpu_threads: Some(16),
        memory_in_mb: Some(2048),
        is_running: Some(false),
        tags: HashMap::new(),
        provider: String::from(PROVIDER_IDENTIFIER),
      }
    ]
  }
}

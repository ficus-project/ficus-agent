use async_trait::async_trait;
use ficus_agent_lib::models::errors::FetchResourceError;
use ficus_agent_lib::models::resources::VirtualMachine;
use ficus_agent_lib::resources::virtual_machines::VirtualMachineProvider;

use crate::mock_data::MockData;


pub struct MockVirtualMachineProvider;


impl MockVirtualMachineProvider {
  pub async fn new() -> Self {
    MockVirtualMachineProvider {}
  }
}

#[async_trait(?Send)]
impl VirtualMachineProvider for MockVirtualMachineProvider {

  async fn list_virtual_machines(&self) -> Result<Box<Vec<VirtualMachine>>, FetchResourceError> {
    Ok(Box::new(MockData::generate_virtual_machines()))
  }
}

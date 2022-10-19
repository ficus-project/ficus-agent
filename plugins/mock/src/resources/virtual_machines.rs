use async_trait::async_trait;
use ficus_agent_lib::models::errors::FetchResourceError;
use ficus_agent_lib::models::resources::VirtualMachine;
use ficus_agent_lib::resources::virtual_machines::VirtualMachineDatasource;

use crate::mock_data::MockData;


pub struct MockVirtualMachineDatasource;


#[async_trait]
impl VirtualMachineDatasource for MockVirtualMachineDatasource {
  async fn new() -> Self {
    MockVirtualMachineDatasource {}
  }

  async fn list_virtual_machines(&self) -> Result<Vec<VirtualMachine>, FetchResourceError> {
    Ok(MockData::generate_virtual_machines())
  }
}

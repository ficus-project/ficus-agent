use aws_config::{meta::region::RegionProviderChain, SdkConfig};


pub async fn load_config() -> SdkConfig {
  let region_provider = RegionProviderChain::default_provider().or_else("eu-west-3");
  aws_config::from_env().region(region_provider).load().await
}

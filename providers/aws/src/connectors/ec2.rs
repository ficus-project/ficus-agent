use aws_sdk_ec2::{Client,model::{Instance, InstanceTypeInfo}};

const EC2_MAX_RESULTS: i32 = 20;


pub async fn get_ec2(client: &Client, next_token: Option<String>) -> Result<(Vec<Instance>, Option<String>), aws_sdk_ec2::Error> {
  let response = client.describe_instances().set_next_token(next_token).max_results(EC2_MAX_RESULTS).send().await?;

  let mut instances: Vec<Instance> = vec![];
  for reservation in response.reservations().unwrap_or_default() {
    instances.append(&mut reservation.instances().unwrap_or_default().to_vec());
  }

  Ok((instances, response.next_token))
}

pub async fn get_instance_types(ec2_client: &Client, next_token: Option<String>) -> Result<(Vec<InstanceTypeInfo>, Option<String>), aws_sdk_ec2::Error> {
  let response = ec2_client.describe_instance_types().set_next_token(next_token).send().await?;
  
  Ok((response.instance_types().unwrap_or_default().to_vec(), response.next_token))
}

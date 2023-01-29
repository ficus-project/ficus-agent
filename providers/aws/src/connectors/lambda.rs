use aws_sdk_lambda::{Client, model::FunctionConfiguration};

const LAMBDA_MAX_RESULTS: i32 = 20;


pub async fn get_lambdas(client: &Client, next_marker: Option<String>) -> Result<(Vec<FunctionConfiguration>, Option<String>), aws_sdk_lambda::Error> {
  let response = client.list_functions().set_marker(next_marker).set_max_items(Some(LAMBDA_MAX_RESULTS)).send().await?;

  Ok((response.functions().unwrap_or_default().to_vec(), response.next_marker().map(|m| String::from(m))))
}

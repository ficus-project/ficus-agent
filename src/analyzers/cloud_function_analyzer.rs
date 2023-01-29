use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use ficus_agent_aws::resources::cloud_functions::AwsCloudFunctionProvider;
use ficus_agent_lib::{models::resources::{CloudFunction, UsageMetric}, resources::cloud_functions::CloudFunctionProvider};

use crate::connectors::influxdb::InfluxdbWriter;


async fn get_cf_providers() -> Vec<Box<dyn CloudFunctionProvider>> {
  vec![
    #[cfg(feature = "aws")]
    Box::new(AwsCloudFunctionProvider::new().await),
  ]
}

pub async fn analyze_cloud_functions(from_timestamp: u64, to_timestamp: u64) {
  let (cloud_functions, cloud_functions_usage) = fetch_cloud_functions(from_timestamp, to_timestamp).await;
  
  store_cfs_existence_data(cloud_functions).await;
  store_cfs_usage_data(cloud_functions_usage).await;
}

async fn fetch_cloud_functions(from_timestamp: u64, to_timestamp: u64) -> (Vec<CloudFunction>, HashMap<String, HashMap<String, UsageMetric>>) {
  let providers = get_cf_providers().await;
  let mut cloud_functions: Vec<CloudFunction> = vec![];
  let mut cloud_functions_usage = HashMap::new();

  for provider in providers {
    let cf_result = provider.list_cloud_functions().await;
    if let Ok(mut cfs) = cf_result {
      cloud_functions.append((*cfs).as_mut());
      
      if let Ok(cloud_function_usage) = provider.measure_cloud_functions_usage(
        (*cloud_functions).iter().filter(|cf| cf.name.is_some()).map(|cf| cf.name.clone().unwrap()).collect::<Vec<String>>(),
        from_timestamp,
        to_timestamp
      ).await {
        cloud_functions_usage.extend(*cloud_function_usage);
      };
    }
  }

  (cloud_functions, cloud_functions_usage)
}

async fn store_cfs_existence_data(cloud_functions: Vec<CloudFunction>) {
  let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
  let influxdb_writer = InfluxdbWriter::new();
  println!("Found {} existing cloud functions", cloud_functions.len());
  for cloud_function in cloud_functions {
    let (tags, fields) = build_tags_and_fields_for_cloud_function(cloud_function).await;

    influxdb_writer.send_metric(String::from("function"), now_timestamp, &tags, &fields).await.unwrap();
  }
}

async fn build_tags_and_fields_for_cloud_function(cf: CloudFunction) -> (HashMap<String, String>, HashMap<String, i64>) {
  let mut tags = HashMap::new();
  if let Some(name) = cf.name { tags.insert(String::from("name"), name); }
  if let Some(description) = cf.description { if !description.is_empty() { tags.insert(String::from("description"), description); } }
  tags.insert(String::from("provider"), String::from(cf.provider));

  let mut int_fields: HashMap<String, i64> = HashMap::new();
  if let Some(code_size) = cf.code_size { int_fields.insert(String::from("code_size"), code_size as i64); }
  if let Some(memory) = cf.memory { int_fields.insert(String::from("memory"), memory as i64); }
  
  (tags, int_fields)
}

async fn store_cfs_usage_data(cfs_usage: HashMap<String, HashMap<String, UsageMetric>>) {
  let mut metric_sent_count: i32 = 0;
  let cfs_count = cfs_usage.len();
  let influxdb_writer = InfluxdbWriter::new();
  for (cf_name, cf_metrics) in cfs_usage {
    for (metric_name, usage) in cf_metrics {
      let mut tags = HashMap::new();
      tags.insert(String::from("name"), cf_name.clone());

      let mut fields = HashMap::new();
      if let Some(average) = usage.average { fields.insert(String::from("average"), average); }
      if let Some(sum) = usage.sum { fields.insert(String::from("sum"), sum); }
      if let Some(timestamp) = usage.timestamp {
        influxdb_writer.send_metric(metric_name.clone(), timestamp, &tags, &fields).await.unwrap();
        metric_sent_count = metric_sent_count + 1;
      }
    }
  }
  println!("Sent {} usage metrics to influx, related to the consumption of {} cloud functions", metric_sent_count, cfs_count);
}

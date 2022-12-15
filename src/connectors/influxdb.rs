use std::collections::HashMap;

use hocon::HoconLoader;
use influxdb2::{Client, models::{DataPoint, FieldValue}, RequestError};
use serde::Deserialize;


const INFLUXDB_CONFIG_PATH: &str = "./.influx.toml";

pub struct InfluxdbWriter {
  influx_client: Client,
  ficus_bucket: String,
}

#[derive(Deserialize, Debug)]
struct InfluxdbConfig {
  pub url: String,
  pub organization: String,
  pub token: String,  
  pub bucket: String,
}

impl InfluxdbWriter {
  pub fn new() -> Self {
    let influx_config = load_config();

    InfluxdbWriter {
      influx_client: Client::new(influx_config.url, influx_config.organization, influx_config.token),
      ficus_bucket: influx_config.bucket,
    }
  }

  pub async fn send_metric<T: Into<FieldValue> + Copy>(&self, metric_name: String, timestamp_sec: i64, tags: &HashMap<String, String>, fields: &HashMap<String, T>) -> Result<(), RequestError> {
    let mut datapoint_builder = DataPoint::builder(metric_name).timestamp(timestamp_sec * 1_000_000_000);
    for (key, value) in &*tags {
      datapoint_builder = datapoint_builder.tag(key, value);
    }
    for (key, value) in &*fields {
      datapoint_builder = datapoint_builder.field(key, *value);
    }
    let datapoint = datapoint_builder.build().unwrap();

    Ok(self.influx_client.write(&self.ficus_bucket, futures::stream::iter(vec![datapoint])).await?)
  }
}

fn load_config() -> InfluxdbConfig {
  HoconLoader::new()
    .load_file(INFLUXDB_CONFIG_PATH)
    .expect("Error when trying to load influxdb config")
    .resolve()
    .expect("Error when trying to deserialize influxdb config")
}

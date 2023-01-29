use std::time::{SystemTime, UNIX_EPOCH};


use super::{virtual_machine_analyzer::analyze_virtual_machines, cloud_function_analyzer::analyze_cloud_functions};


const DAY_DURATION_SEC: u64 = 24 * 60 * 60;
const WEEK_DURATION_SEC: u64 = DAY_DURATION_SEC * 7;

pub async fn analyze_resources() {
  let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
  let past_day_timestamp = now_timestamp - WEEK_DURATION_SEC;

  analyze_virtual_machines(past_day_timestamp, now_timestamp).await;
  analyze_cloud_functions(past_day_timestamp, now_timestamp).await;
}

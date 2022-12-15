
#[derive(Debug)]
pub struct VirtualMachine {
  pub identifier: Option<String>,
  pub cpu_cores: Option<i32>,
  pub cpu_threads: Option<i32>,
  pub memory_in_mb: Option<i64>,
  pub is_running: Option<bool>,
}

#[derive(Debug)]
pub struct UsageMetric {
  pub timestamp: Option<i64>,
  pub average: Option<f64>,
  pub sum: Option<f64>,
}

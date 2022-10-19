
#[derive(Debug)]
pub struct VirtualMachine {
  pub identifier: Option<String>,
  pub cpu_core: Option<i32>,
  pub memory_in_mb: Option<i64>
}

#[derive(Debug)]
pub struct UsageMetric {
  pub timestamp: Option<i64>,
  pub average: Option<f64>,
  pub sum: Option<f64>,
}

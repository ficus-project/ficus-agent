use aws_sdk_cloudwatch::{Client, types::DateTime, model::{Dimension, Statistic, StandardUnit, Datapoint}};


pub async fn get_ec2_cpu_usage(client: &Client, instance_id: &String, start_time: DateTime, end_time: DateTime, period: i32) -> Result<Vec<Datapoint>, aws_sdk_cloudwatch::Error> {
  let response = client
    .get_metric_statistics()
    .set_unit(Some(StandardUnit::Percent))
    .set_namespace(Some(String::from("AWS/EC2")))
    .set_metric_name(Some(String::from("CPUUtilization")))
    .set_period(Some(period))
    .set_start_time(Some(start_time))
    .set_end_time(Some(end_time))
    .set_statistics(Some(vec![Statistic::Sum, Statistic::Average]))
    .set_dimensions(Some(vec![
      Dimension::builder().set_name(Some(String::from("InstanceId"))).set_value(Some(instance_id.clone())).build()
    ]))
    .send().await?;
  println!("{:?}", response);

  Ok(response.datapoints().unwrap_or_default().to_vec())
}

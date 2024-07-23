use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use std::error::Error;

pub const REGION: BucketLocationConstraint = BucketLocationConstraint::UsWest2;

pub fn s3_client(conf: &aws_config::SdkConfig) -> aws_sdk_s3::Client {
    let mut s3_config_builder = aws_sdk_s3::config::Builder::from(conf);
    s3_config_builder.set_force_path_style(Some(true));
    aws_sdk_s3::Client::from_conf(s3_config_builder.build())
}

pub async fn create_s3_bucket(client: &aws_sdk_s3::Client, bucket_name: &str) -> Result<(), Box<dyn Error>> {
    let create_bucket_configuration = CreateBucketConfiguration::builder()
        .location_constraint(REGION)
        .build();

    client.create_bucket()
        .bucket(bucket_name)
        .create_bucket_configuration(create_bucket_configuration)
        .send()
        .await?;

    println!("Created bucket: {}", bucket_name);
    Ok(())
}

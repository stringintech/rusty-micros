use aws_config::BehaviorVersion;
use aws_config::Region;
use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use std::error::Error;

const LOCALSTACK_ENDPOINT: &str = "http://localhost:4566/";
const REGION: BucketLocationConstraint = BucketLocationConstraint::UsWest2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Manually create AWS configuration
    let region = Region::new(REGION.as_str());

    let shared_config = SdkConfig::builder()
        .region(region)
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            "dummy-access-key-id",
            "dummy-secret-access-key",
            None,
            None,
            "default",
        )))
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(LOCALSTACK_ENDPOINT)
        .build();

    let s3_client = s3_client(&shared_config);

    // Create an S3 bucket
    create_s3_bucket(&s3_client, "my-test-bucket").await?;

    // List all S3 buckets
    let resp = s3_client.list_buckets().send().await?;
    let buckets = resp.buckets();
    let num_buckets = buckets.len();

    println!("Buckets:");
    for bucket in buckets {
        println!("  {}", bucket.name().unwrap_or_default());
    }

    println!("\nFound {} buckets.\n", num_buckets);

    Ok(())
}

fn s3_client(conf: &aws_config::SdkConfig) -> aws_sdk_s3::Client {
    let mut s3_config_builder = aws_sdk_s3::config::Builder::from(conf);
    s3_config_builder.set_force_path_style(Some(true));
    aws_sdk_s3::Client::from_conf(s3_config_builder.build())
}

async fn create_s3_bucket(client: &aws_sdk_s3::Client, bucket_name: &str) -> Result<(), Box<dyn Error>> {
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

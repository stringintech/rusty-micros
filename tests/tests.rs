use aws_config::BehaviorVersion;
use aws_config::Region;
use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use std::error::Error;
use rusty_micros::*;
use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::AsyncRunner, GenericImage};

#[tokio::test]
async fn test_s3_integration() -> Result<(), Box<dyn Error>> {
    // Start LocalStack container for S3 integration testing
    let container = GenericImage::new("localstack/localstack", "3.5.0")
        .with_exposed_port(4566.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready."))
        .start()
        .await?;
    
    // Get the host port where the LocalStack container is accessible
    let host_port = container.get_host_port_ipv4(4566.tcp()).await?;
    let url = format!("http://localhost:{host_port}");

    // Set up AWS SDK configuration with dummy credentials and LocalStack endpoint
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
        .endpoint_url(url)
        .build();

    // Create S3 client using the configured settings
    let s3_client = s3_client(&shared_config);

    // Verify no buckets exist initially
    let num_buckets = s3_client.list_buckets().send().await?.buckets().len();
    assert_eq!(0, num_buckets);

    // Create a new S3 bucket for testing
    create_s3_bucket(&s3_client, "my-test-bucket").await?;

    // Verify that the bucket was created successfully
    let num_buckets = s3_client.list_buckets().send().await?.buckets().len();
    assert_eq!(1, num_buckets);

    Ok(())
}

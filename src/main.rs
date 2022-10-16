mod aws;

use aws_sdk_s3 as s3;
use aws_types::Credentials;
use http::Uri;
use std::env;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result {
    let bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "test-bucket".to_string());

    let client = client();
    println!("Created client");

    println!("Writing order to S3");
    let order = aws::order::Order::create();
    aws::writer::upload_to_s3(&client, &bucket_name, order).await?;

    let orders = aws::reader::get_orders(&client, &bucket_name).await?;
    println!("Found {} orders in S3 bucket", orders.len());

    for order in orders {
        println!("Printing order price");
        println!("{}", order.total_price());
    }

    Ok(())
}

fn client() -> s3::Client {
    let conf = s3::Config::builder()
        .region(s3::Region::new("us-east-1"))
        .endpoint_resolver(s3::Endpoint::immutable(Uri::from_static(
            "http://localstack:4566",
        )))
        .credentials_provider(Credentials::from_keys("access-key", "secret-key", None))
        .build();
    s3::Client::from_conf(conf)
}

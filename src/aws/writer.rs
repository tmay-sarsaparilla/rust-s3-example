use crate::aws::order::Order;
use aws_sdk_s3::{types::ByteStream, Client};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn upload_to_s3(client: &Client, bucket_name: &str, order: Order) -> Result {
    let bytes = bincode::serialize(&order)?;
    let body = ByteStream::from(bytes);
    client
        .put_object()
        .bucket(bucket_name)
        .key("file.json")
        .body(body)
        .send()
        .await?;

    Ok(())
}

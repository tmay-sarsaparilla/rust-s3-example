use aws_sdk_s3::{Client, types::ByteStream};

use crate::order::Order;

pub async fn upload_to_s3(client: &Client, order: Order) {
    let bytes = bincode::serialize(&order).unwrap();
    let body = ByteStream::from(bytes);
    let _response = client.put_object()
                          .bucket("test-bucket")
                          .key("file.json")
                          .body(body)
                          .send()
                          .await;
}

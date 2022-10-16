use aws_sdk_s3::model::Object;
use aws_sdk_s3::Client;
use futures::future;

use crate::aws::order::Order;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_orders(client: &Client, bucket_name: &str) -> Result<Vec<Order>> {
    let response = client.list_objects_v2().bucket(bucket_name).send().await?;

    let objects = match response.contents() {
        Some(objects) => objects,
        None => return Ok(Vec::new()),
    };

    let orders = future::try_join_all(
        objects
            .iter()
            .map(|o: &Object| get_order_from_object(client, o)),
    )
    .await?
    .into_iter()
    .flatten()
    .collect();

    Ok(orders)
}

async fn get_order_from_object(client: &Client, object: &Object) -> Result<Option<Order>> {
    let key = match object.key() {
        Some(key) => key,
        None => return Ok(None),
    };
    let object_response = client
        .get_object()
        .bucket("test-bucket")
        .key(key)
        .send()
        .await?;

    let body = object_response.body.collect().await?;
    let order: Order = bincode::deserialize(&body.into_bytes())?;
    Ok(Some(order))
}

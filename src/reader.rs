use aws_sdk_s3::Client;

use crate::order::Order;

pub async fn get_orders(client: &Client) -> Result<Vec<Order>, aws_sdk_s3::Error> {
    let response = client.list_objects_v2()
                         .bucket("test-bucket")
                         .send()
                         .await?;

    let objects = response.contents().unwrap();
    let mut orders: Vec<Order> = Vec::new();
    for object in objects.iter() {
        let key = object.key().unwrap();
        let order = get_order_from_key(&client, key).await?;
        orders.push(order);
    }

    Ok(orders)
}

async fn get_order_from_key(client: &Client, key: &str) -> Result<Order, aws_sdk_s3::Error> {
    let object_response = client.get_object()
        .bucket("test-bucket")
        .key(key)
        .send()
        .await?;

    let body = match object_response.body.collect().await {
        Ok(body) => body,
        Err(error) => panic!("Reading object bytes failed: {:?}", error)
    };
    let order: Order = bincode::deserialize(&body.into_bytes()).unwrap();
    Ok(order)
}

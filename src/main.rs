mod writer;
mod order;
mod reader;

use http::Uri;
use aws_sdk_s3 as s3;
use aws_types::Credentials;

#[tokio::main]
async fn main() {
     let conf = s3::Config::builder()
          .region(s3::Region::new("us-east-1"))
          .endpoint_resolver(
               s3::Endpoint::immutable(Uri::from_static("http://localhost:4566"))
          )
          .credentials_provider(Credentials::from_keys("access-key", "secret-key", None))
          .build();
     let client = s3::Client::from_conf(conf);
     println!("Created client");

     println!("Writing order to S3");
     let order = order::Order::create();
     writer::upload_to_s3(&client, order).await;

     let orders = match reader::get_orders(&client).await {
          Ok(orders) => orders,
          Err(error) => panic!("Retrieval of orders failed: {:?}", error)
     };

     println!("Found {} orders in S3 bucket", orders.len());

     for order in orders {
          println!("Printing order price");
          println!("{}", order.total_price());
     }
}

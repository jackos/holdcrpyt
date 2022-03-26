// use aws_sdk_dynamodb::Client;
// use aws_sdk_dynamodb::model::AttributeValue;
// use serde::Deserialize;
// use serde_json::{json, Value};
// use uuid::Uuid;
// use lambda_runtime::{service_fn, LambdaEvent, Error};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let func = service_fn(func);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// async fn func(event: LambdaEvent<Person>) -> Result<Value, Error> {
//     let (event, _context) = event.into_parts();

//     let uuid = Uuid::new_v4().to_string();
//     let config = aws_config::load_from_env().await;
//     let client = Client::new(&config); 

//     let request = client.put_item()
//         .table_name("user")
//         .item("uid", AttributeValue::S(String::from(uuid)))
//         .item("first_name", AttributeValue::S(String::from(event.first_name)))
//         .item("last_name", AttributeValue::S(String::from(event.last_name)));
//     request.send().await?;

//     Ok(json!({"message": "record written"}))
// }


// #[derive(Deserialize)]
// struct Person {
//     first_name: String,
//     last_name: String
// }

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}

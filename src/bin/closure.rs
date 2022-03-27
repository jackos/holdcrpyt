use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;
use lambda_http::{service_fn, Error, IntoResponse, Request, Response, Body};

#[derive(Deserialize, Debug)]
struct Person {
    first_name: String,
    last_name: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(|request: lambda_http::Request| async {
    let (_, body) = request.into_parts();
    if let Body::Text(txt) = body {
        let b: Person = serde_json::from_str(&txt)?;
        
        let uuid = Uuid::new_v4().to_string();
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config); 
        
        let request = client.put_item()
        .table_name("user")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("first_name", AttributeValue::S(String::from(b.first_name)))
        .item("last_name", AttributeValue::S(String::from(b.last_name)));
        request.send().await?;
        
        return Ok(json!({"message": "record written"}));
    }

    Ok(Value::Null)

    })).await?;
    Ok(())
}

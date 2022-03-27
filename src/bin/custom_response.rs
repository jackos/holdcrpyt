use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::Deserialize;
use serde_json::{json};
use uuid::Uuid;
use lambda_http::{service_fn, Error, IntoResponse, Request, Response,  Body, Service};
use lambda_runtime::tower::{ServiceExt, BoxError};


#[derive(Deserialize, Debug)]
struct Person {
    first_name: String,
    last_name: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handle));
    Ok(())
}


async fn handle(request: Request) -> Result<Response<String>, BoxError> {
    let response = Response::new("Hello, World!".to_string());
    Ok(response)
}

// let mut service = service_fn(handle);

// let response = service
//     .ready()
//     .await?
//     .call(Request::new())
//     .await?;

// assert_eq!("Hello, World!", response.into_body());

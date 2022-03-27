use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize};
use lambda_http::{service_fn, Error, IntoResponse, Request, Body};
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;

use holdcrypt::Res;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    first_name: String,
    last_name: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    lambda_http::run(service_fn(lambda)).await?;
    Ok(())
}


async fn lambda(event: Request) -> Result<impl IntoResponse, Error> {
    let body = event.body();

    if let Body::Text(txt) = body {
        let user: User = match serde_json::from_str(&txt) {
            Ok(val) => val,
            Err(err) => return Ok(Res::parse_body_error(err)),
        };
        
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config); 
        let request = client.put_item()
        .table_name("user")
        .item("username", AttributeValue::S(user.username))
        .item("first_name", AttributeValue::S(user.first_name))
        .item("last_name", AttributeValue::S(user.last_name))
        .item("transactions", AttributeValue::L([].to_vec()));

        if let Err(err) = request.send().await {
            return Ok(Res::internal_server_error("failed to add user to dynamodb", Box::new(err)));
        }

        return Ok(Res::ok("successfully added user"))
    }

    Ok(Res::bad_request("must include a body"))
}

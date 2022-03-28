//! Add a user or update if the user already exists

use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, IntoResponse, Request};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::{Error, Res, UserPutRequest};

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
        let user: UserPutRequest = match serde_json::from_str(txt) {
            Ok(val) => val,
            Err(err) => return Ok(Res::parse_body_error(err)),
        };

        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        let request = client
            .put_item()
            .table_name("user")
            .item("username", AttributeValue::S(user.username))
            .item("first_name", AttributeValue::S(user.first_name))
            .item("last_name", AttributeValue::S(user.last_name))
            .item("transactions", AttributeValue::L([].to_vec()));

        if let Err(err) = request.send().await {
            return Ok(Res::internal_server_error(
                "failed to add user to dynamodb",
                Box::new(err),
            ));
        }

        return Ok(Res::ok("successfully added user"));
    }

    Ok(Res::bad_request("must include a body"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Body;
    use serde_json::json;

    #[tokio::test]
    async fn put_user() {
        let body = UserPutRequest {
            first_name: "test".to_string(),
            last_name: "user".to_string(),
            username: "testuser".to_string(),
        };

        let body = serde_json::to_string(&body).expect("failed to serialize to json string");

        let request = Request::new(Body::Text(body));

        let response = lambda(request)
            .await
            .expect("failed to run lambda")
            .into_response();

        assert_eq!(
            response.body(),
            json!({"message": "successfully added user"})
                .into_response()
                .body()
        );

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn fail_to_put_empty_username() {
        let body = UserPutRequest {
            first_name: "test".to_string(),
            last_name: "user".to_string(),
            username: "".to_string(),
        };

        let body = serde_json::to_string(&body).expect("failed to serialize to json string");

        let request = Request::new(Body::Text(body));

        let response = lambda(request)
            .await
            .expect("failed to run lambda")
            .into_response();

        assert_eq!(response.status(), 500);
    }
}

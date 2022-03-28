//! Add a transaction, which is stored in the user table under transactions

use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, IntoResponse, Request};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::{Error, Res, Transaction};

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
        let trans: Transaction = match serde_json::from_str(txt) {
            Ok(val) => val,
            Err(err) => return Ok(Res::parse_body_error(err)),
        };

        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        let map = HashMap::from([
            ("coin".to_string(), AttributeValue::S(trans.coin)),
            (
                "amount".to_string(),
                AttributeValue::N(trans.amount.to_string()),
            ),
            (
                "price".to_string(),
                AttributeValue::N(trans.price.to_string()),
            ),
        ]);

        let request = client
            .update_item()
            .table_name("user")
            .key("username", AttributeValue::S(trans.username.clone()))
            .condition_expression("username = :username")
            .update_expression(
                "set transactions = list_append(if_not_exists(transactions, :trans), :trans)",
            )
            .expression_attribute_values(":trans", AttributeValue::L(vec![AttributeValue::M(map)]))
            .expression_attribute_values(":username", AttributeValue::S(trans.username));

        if let Err(err) = request.send().await {
            return Ok(Res::internal_server_error(
                "failed to add transaction to dynamodb",
                Box::new(err),
            ));
        }

        return Ok(Res::ok("successfully added transaction"));
        // return Ok(json!({"message": "record written"}));
    }

    Ok(Res::bad_request("must include a body"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Body;
    use serde_json::json;

    #[tokio::test]
    async fn post_transaction() {
        let body = Transaction {
            username: "testuser".to_string(),
            coin: "Ethereum".to_string(),
            amount: 10.10,
            price: 12.5,
        };

        let body = serde_json::to_string(&body).expect("failed to serialize to json string");

        let request = Request::new(Body::Text(body));

        let response = lambda(request)
            .await
            .expect("failed to run lambda")
            .into_response();

        assert_eq!(
            response.body(),
            json!({"message": "successfully added transaction"})
                .into_response()
                .body()
        );

        assert_eq!(response.status(), 200);
    }
}

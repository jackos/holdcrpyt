//! Gets all available coins and prices from dynamodb

use aws_sdk_dynamodb::{output::ScanOutput, Client};
use lambda_http::{service_fn, IntoResponse, Request};
use std::{collections::HashMap, io::ErrorKind};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::{CoinPrice, Error, Res};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    lambda_http::run(service_fn(lambda)).await?;
    Ok(())
}

async fn lambda(_: Request) -> Result<impl IntoResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let items = match client.scan().table_name("coin").send().await {
        Ok(v) => v,
        Err(error) => {
            return Ok(Res::internal_server_error(
                "dynamodb table doesn't exist",
                Box::new(error),
            ))
        }
    };

    let price_map = match get_items(items) {
        Ok(v) => v,
        Err(error) => {
            return Ok(Res::internal_server_error(
                "failed to scan result from dynamodb",
                error,
            ))
        }
    };

    let res_body = match serde_json::to_string(&price_map) {
        Ok(v) => v,
        Err(error) => {
            return Ok(Res::internal_server_error(
                "failed to parse dynamodb response",
                Box::new(error),
            ))
        }
    };
    Ok(Res::ok_body(&res_body))
}

fn get_items(items: ScanOutput) -> Result<HashMap<String, CoinPrice>, Error> {
    let mut price_map = HashMap::new();
    for map in items.items().ok_or("no items in table")? {
        let name = map
            .get("name")
            .ok_or("name key doesn't exist")?
            .as_s()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "name type incorrect"))?;
        let symbol = map
            .get("symbol")
            .ok_or("symbol key doesn't exist")?
            .as_s()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "symbol type incorrect"))?;
        let price = map
            .get("price")
            .ok_or("price key doesn't exist")?
            .as_n()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "price type incorrect"))?
            .parse::<f64>()?;
        price_map.insert(
            symbol.clone(),
            CoinPrice {
                name: name.clone(),
                price,
            },
        );
    }
    Ok(price_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Body;

    #[tokio::test]
    async fn get_coins_parses_contains_values() {
        let request = Request::default();
        let response = lambda(request)
            .await
            .expect("failed to get coins")
            .into_response();

        let coins: HashMap<String, CoinPrice> = match response.body() {
            Body::Text(v) => serde_json::from_str(v).expect("failed to parse response body"),
            _ => panic!("response body not text"),
        };

        for (key, value) in coins.iter() {
            assert!(!key.is_empty());
            assert!(!value.name.is_empty());
            assert!(value.price > 0.0);
        }

        assert_eq!(response.status(), 200);
    }
}

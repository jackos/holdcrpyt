//! Queries the market for each coin passed into the body and retrieves
//! the most recent 50 asks, and takes the average of those to determine
//! the price

use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, IntoResponse, Request};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::{BinancePrices, CoinPrice, CoinsPutRequest, Error, Res};

const BINANCE_PRICES_URL: &str = "https://api.binance.com/api/v3/depth";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    lambda_http::run(service_fn(lambda)).await?;
    Ok(())
}

async fn lambda(event: Request) -> Result<impl IntoResponse, Error> {
    let mut price_map: HashMap<String, CoinPrice> = HashMap::new();

    let body: CoinsPutRequest = match event.body() {
        Body::Text(text) => match serde_json::from_str(text) {
            Ok(js) => js,
            Err(err) => return Ok(Res::parse_body_error(err)),
        },
        Body::Empty => return Ok(Res::bad_request("no body provided")),
        Body::Binary(_) => return Ok(Res::bad_request("binary body not supported")),
    };

    for coin in body.coins {
        let params = [("symbol", &coin.symbol), ("limit", &"50".to_string())];
        let client = reqwest::Client::new();
        let config = aws_config::load_from_env().await;
        let dynamo_client = Client::new(&config);
        let mut res = match client.get(BINANCE_PRICES_URL).query(&params).send() {
            Ok(res) => res,
            Err(err) => {
                return Ok(Res::internal_server_error(
                    "failed to get prices from Binance",
                    Box::new(err),
                ))
            }
        };

        let prices: BinancePrices = match res.json() {
            Ok(res) => res,
            Err(err) => {
                return Ok(Res::parse_response_error(
                    "failed to parse prices from Binance",
                    err,
                ))
            }
        };

        let mut sum = 0f64;
        let total: f64 = prices.asks.len() as f64;
        for price in prices.asks {
            sum += price[0]
                .parse::<f64>()
                .expect("failed to parse price as f64");
        }
        let average = sum / total;
        price_map.insert(
            coin.symbol.clone(),
            CoinPrice {
                price: average,
                name: coin.name.clone(),
            },
        );
        let request = dynamo_client
            .put_item()
            .table_name("coin")
            .item("name", AttributeValue::S(coin.name))
            .item("symbol", AttributeValue::S(coin.symbol))
            .item("price", AttributeValue::N(average.to_string()));

        if let Err(err) = request.send().await {
            return Ok(Res::internal_server_error(
                "failed to add coin to dynamodb",
                Box::new(err),
            ));
        }
    }

    let resp = match serde_json::to_string(&price_map) {
        Ok(v) => v,
        Err(error) => {
            return Ok(Res::internal_server_error(
                "failed to convert struct to json string",
                Box::new(error),
            ))
        }
    };
    Ok(Res::ok_body(&resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use holdcrypt::CoinPutRequest;
    use lambda_http::Body;

    #[tokio::test]
    async fn put_coin_response_non_empty() {
        let body = CoinsPutRequest {
            coins: vec![CoinPutRequest {
                name: "Ethereum".to_string(),
                symbol: "ETHAUD".to_string(),
            }],
        };

        let body = serde_json::to_string(&body).expect("failed to serialize to json string");

        let request = Request::new(Body::Text(body));

        let response = lambda(request)
            .await
            .expect("failed to run lambda")
            .into_response();

        let coins: HashMap<String, CoinPrice> = match response.body() {
            Body::Text(v) => serde_json::from_str(v).expect("failed to parse response body"),
            _ => panic!("response body not text"),
        };

        dbg!(&coins);

        for (key, value) in coins.iter() {
            assert!(!key.is_empty());
            assert!(!value.name.is_empty());
            assert!(value.price > 0.0);
        }

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn fail_to_put_empty_coin() {
        let body = CoinsPutRequest {
            coins: vec![CoinPutRequest {
                name: "".to_string(),
                symbol: "".to_string(),
            }],
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

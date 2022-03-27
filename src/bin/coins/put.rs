use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Error, Body, IntoResponse, Request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::Res;

const URL: &str = "https://api.binance.com/api/v3/depth";

// Binance response, bids and asks contain pairs in array format: [price, amount]
#[derive(Serialize, Deserialize)]
struct Prices {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    pub bids: Vec<Vec<String>>, // offered buy prices
    pub asks: Vec<Vec<String>>, // offered sell prices
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    coins: Vec<Coin>,
}

#[derive(Serialize, Deserialize)]
struct Coin {
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize)]
struct CoinValue {
    name: String,
    price: f64,
}

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
    let mut price_map: HashMap<String, CoinValue> = HashMap::new();

    let body: RequestBody = match event.body() {
        Body::Text(text) => match serde_json::from_str(text) {
            Ok(js) => js,
            Err(err) => return Ok(Res::parse_body_error(err)),
        },
        Body::Empty => return Ok(Res::bad_request("no body provided")),
        Body::Binary(_) => return Ok(Res::bad_request("binary body not supported"))
    };

    for coin in body.coins {
        let params = [("symbol", &coin.symbol), ("limit", &"50".to_string())];
        let client = reqwest::Client::new();
        let config = aws_config::load_from_env().await;
        let dynamo_client = Client::new(&config); 
        let mut res = match client
                    .get(URL)
                    .query(&params)
                    .send() {
            Ok(res) => res,
            Err(err) => return Ok(Res::internal_server_error("failed to get prices from Binance", Box::new(err))),
        };

        let prices: Prices = match res.json() {
            Ok(res) => res,
            Err(err) => return Ok(Res::parse_response_error("failed to parse prices from Binance", err)),
        };

        let mut sum = 0f64;
        let total: f64 = prices.asks.len() as f64;
        for price in prices.asks {
            sum += price[0].parse::<f64>().expect("failed to parse price as f64");
        }
        let average = sum / total;
        price_map.insert(coin.symbol.clone(), CoinValue{price: average, name: coin.name.clone()});
        let request = dynamo_client.put_item()
            .table_name("coin")
            .item("name", AttributeValue::S(coin.name))
            .item("symbol", AttributeValue::S(coin.symbol))
            .item("price", AttributeValue::N(average.to_string()));

        if let Err(err) = request.send().await {
            return Ok(Res::internal_server_error("failed to add coin to dynamodb", Box::new(err)));
        }
    }

    let resp = match serde_json::to_string(&price_map) {
        Ok(v) => v,
        Err(error) => return Ok(Res::internal_server_error("failed to convert struct to json string", Box::new(error))),
    };
    Ok(Res::ok_body(&resp))
}
    
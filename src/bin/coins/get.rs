use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Error, IntoResponse, Request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::Res;

// Binance response, bids and asks contain pairs in array format: [price, amount]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prices {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    pub bids: Vec<Vec<String>>, // offered buy prices
    pub asks: Vec<Vec<String>>, // offered sell prices
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

async fn lambda(_: Request) -> Result<impl IntoResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let items = client.scan().table_name("coin").send().await?;

    let mut price_map = HashMap::new();
    for map in items.items().unwrap() {
        let name = map.get("name").unwrap();
        let name = name.as_s().unwrap();

        let price = map.get("price").unwrap();
        let price: f64 = price.as_n().unwrap().parse::<f64>().unwrap();
        price_map.insert(name, price);
    }
    let res_body = serde_json::to_string(&price_map).unwrap();
    Ok(Res::ok_body(&res_body))
}

#![feature(map_try_insert)]

use std::collections::{HashMap};

use aws_sdk_dynamodb::Client;
use serde::{ Serialize};
use lambda_http::{service_fn, Error, IntoResponse, Request};
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;

use holdcrypt::Res;

#[derive(Serialize)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
	coins: Vec<Coin>,
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

#[derive(Serialize)]
struct Coin {
    name: String,
    symbol: String,
    price: f64,
    amount: f64,
}

async fn lambda(_: Request) -> Result<impl IntoResponse, Error> {
	let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let items = client.scan().table_name("user").send().await?;
    let coin_table = client.scan().table_name("coin").send().await?;
    
	let mut users = vec![];
    for map in items.items().unwrap() {
        let first_name = map.get("first_name").unwrap().as_s().unwrap();
        let last_name = map.get("last_name").unwrap().as_s().unwrap();
        let username = map.get("username").unwrap().as_s().unwrap();
        
		let transactions = map.get("transactions").unwrap().as_l().unwrap();
		let mut coins = HashMap::new();
		for transaction in transactions {
            let trans_map = transaction.as_m().unwrap();
			let amount = trans_map.get("amount").unwrap().as_n().unwrap().parse::<f64>().unwrap();
			let coin = trans_map.get("coin").unwrap().as_s().unwrap();
			coins.entry(coin.clone()).or_insert(0.0);
			coins.insert(coin.clone(), amount + coins[coin]);
		}

        let mut coin_vec = Vec::new();
        for coin in coin_table.items().unwrap() {
            let symbol = coin.get("symbol").unwrap().as_s().unwrap();
            if let Some(amount) = coins.get(symbol) {
                let price = coin.get("price").unwrap().as_n().unwrap().parse::<f64>().unwrap();
                let name = coin.get("name").unwrap().as_s().unwrap();
                let coin_value = Coin{name: name.clone(), price: price, symbol: symbol.clone(), amount: *amount };
                coin_vec.push(coin_value)
            }
        }
        
        let user = User{first_name: first_name.clone(), last_name: last_name.clone(), username: username.clone(), coins: coin_vec};
		users.push(user);
    }

    let res_body = serde_json::to_string(&users).unwrap();
    Ok(Res::ok_body(&res_body))
}

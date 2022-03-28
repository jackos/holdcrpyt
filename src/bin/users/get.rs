//! Get an array of users with the sum of all the coins, conveniently structured
//! for minimal effort on the frontend

#![feature(map_try_insert)]

use std::collections::HashMap;

use aws_sdk_dynamodb::{output::ScanOutput, Client};
use lambda_http::{service_fn, IntoResponse, Request};
use std::io::ErrorKind;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use holdcrypt::{Coin, Error, Res, UserGetResponse};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    lambda_http::run(service_fn(lambda)).await?;

    Ok(())
}

async fn lambda(_: Request) -> Result<impl IntoResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let user_table = client.scan().table_name("user").send().await?;
    let coin_table = client.scan().table_name("coin").send().await?;

    let users = match get_users(user_table, coin_table) {
        Ok(v) => v,
        Err(error) => {
            return Ok(Res::internal_server_error(
                "failed to parse dynamodb response",
                error,
            ))
        }
    };
    let res_body = match serde_json::to_string(&users) {
        Ok(v) => v,
        Err(error) => {
            return Ok(Res::internal_server_error(
                "failed to convert struct to json string",
                Box::new(error),
            ))
        }
    };
    Ok(Res::ok_body(&res_body))
}

// Returns a generic error, this allows use of the `?` operator so any errors related to dynamodb
// we can return to the caller with the correct status code in the calling code
fn get_users(
    user_table: ScanOutput,
    coin_table: ScanOutput,
) -> Result<Vec<UserGetResponse>, Error> {
    let mut users = vec![];
    for map in user_table.items().ok_or("no users in table")? {
        let first_name = map
            .get("first_name")
            .ok_or("first_name key doesn't exist")?
            .as_s()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "first_name not of type string"))?;
        let last_name = map
            .get("last_name")
            .ok_or("last_name key doesn't exist")?
            .as_s()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "last_name not of type string"))?;
        let username = map
            .get("username")
            .ok_or("username key doesn't exist")?
            .as_s()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "username not of type string"))?;

        let transactions = map
            .get("transactions")
            .ok_or("transactions key doesn't exist")?
            .as_l()
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "transactions type incorrect"))?;

        let mut coins = HashMap::new();
        for transaction in transactions {
            let trans_map = transaction
                .as_m()
                .map_err(|_| std::io::Error::new(ErrorKind::Other, "trans_map type incorrect"))?;
            let amount = trans_map
                .get("amount")
                .ok_or("amount key doesn't exist")?
                .as_n()
                .map_err(|_| std::io::Error::new(ErrorKind::Other, "amount not of type number"))?
                .parse::<f64>()?;
            let coin = trans_map
                .get("coin")
                .ok_or("coin key doesn't exist")?
                .as_s()
                .map_err(|_| std::io::Error::new(ErrorKind::Other, "coin not of type string"))?;

            coins.entry(coin.clone()).or_insert(0.0);
            coins.insert(coin.clone(), amount + coins[coin]);
        }

        let mut coin_vec = Vec::new();
        for coin in coin_table.items().ok_or("no coins in table")? {
            let symbol = coin
                .get("symbol")
                .ok_or("symbol key doesn't exist")?
                .as_s()
                .map_err(|_| std::io::Error::new(ErrorKind::Other, "symbol not of type string"))?;
            if let Some(amount) = coins.get(symbol) {
                let price = coin
                    .get("price")
                    .ok_or("price key doesn't exist")?
                    .as_n()
                    .map_err(|_| std::io::Error::new(ErrorKind::Other, "price not of type number"))?
                    .parse::<f64>()?;
                let name = coin
                    .get("name")
                    .ok_or("name key doesn't exist")?
                    .as_s()
                    .map_err(|_| {
                        std::io::Error::new(ErrorKind::Other, "name not of type string")
                    })?;
                let coin_value = Coin {
                    name: name.clone(),
                    price,
                    symbol: symbol.clone(),
                    amount: *amount,
                };
                coin_vec.push(coin_value)
            }
        }

        let user = UserGetResponse {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            username: username.clone(),
            coins: coin_vec,
        };
        users.push(user);
    }
    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Body;

    #[tokio::test]
    async fn get_users_parses_and_non_empty() {
        let request = Request::default();
        let response = lambda(request)
            .await
            .expect("failed to get users")
            .into_response();

        let users: Vec<UserGetResponse> = match response.body() {
            Body::Text(v) => serde_json::from_str(v).expect("failed to parse response body"),
            _ => panic!("response body not text"),
        };

        for user in users {
            assert!(!user.first_name.is_empty());
            assert!(!user.last_name.is_empty());
        }

        assert_eq!(response.status(), 200);
    }
}

//! A coin is a crypto currency denomination, with prices and amounts.
//! `symbol` is used to query market data e.g. `ETHAUD` `name` is used as a display name e.g. `Ethereum`
use serde::{Deserialize, Serialize};

use super::is_default;

/// All stored data for a coin, can be used with just name or symbol
/// if either doesn't exist when deserialized it will use default values which is an Empty String.
/// When serialized if name or symbol doesn't exist, it will be omitted from the json
#[derive(Serialize, Deserialize, Debug)]
pub struct Coin {
    // The name of the coin for display purposes e.g. `Ethereum`
    #[serde(skip_serializing_if = "is_default", default)]
    pub name: String,
    // The symbol of the coin for looking it up on markets e.g. `ETHAUD`
    #[serde(skip_serializing_if = "is_default", default)]
    pub symbol: String,
    // The price of the coin at the time of transaction
    pub price: f64,
    // The amount of coins for a transaction or total coins owned by a user
    pub amount: f64,
}

/// Used inside maps where a coin symbol will map to a price and full name
/// e.g. ETHAUD: CoinPrice{name: "Ethereum", price: 4500.50}
#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub name: String,
    pub price: f64,
}

/// Used to deserialize a put request for coins, the lambda takes care of finding
/// the average ask price and storing it in dynamodb
#[derive(Serialize, Deserialize)]
pub struct CoinsPutRequest {
    pub coins: Vec<CoinPutRequest>,
}

/// The name is stored in dynamodb for a display name when being displayed on a frontend
/// the symbol is used to lookup the coin ask prices on the crypto market
#[derive(Serialize, Deserialize)]
pub struct CoinPutRequest {
    pub name: String,
    pub symbol: String,
}

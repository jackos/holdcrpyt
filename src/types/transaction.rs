//! An addition or subtraction of coins from a user, total balance is retrieved event sourcing style
//! where you can't delete transactions
use serde::{Deserialize, Serialize};

/// represents a transaction that is added to a user
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub username: String,
    pub coin: String,
    pub amount: f64,
    pub price: f64,
}

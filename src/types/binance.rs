//! Used to get current market value of different coins
use serde::{Deserialize, Serialize};

/// For deserializing a response from Binance for current coin market prices (bids and asks)
#[derive(Serialize, Deserialize)]
pub struct BinancePrices {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    /// contain pairs in array format: [[price, amount], [price, amount]....]
    pub bids: Vec<Vec<String>>,
    /// contain pairs in array format: [[price, amount], [price, amount]....]
    pub asks: Vec<Vec<String>>,
}

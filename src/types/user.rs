//! Represents an owner of crypto assets
use serde::{Deserialize, Serialize};

use super::Coin;

/// Each user contains a a vector of how many coins they own
/// with the total amount and display name. This minimizes the
/// work that needs to be done on the frontend
#[derive(Serialize, Deserialize, Debug)]
pub struct UserGetResponse {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub coins: Vec<Coin>,
}

///  Adds a user to dynamodb, if the username already exists it just
/// updates the first_name and last_name
#[derive(Serialize, Deserialize)]
pub struct UserPutRequest {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

//! Contains serde types that can be used across different lambdas, separated into different
//! modules by domain for easier navigation
pub mod coin;
pub use coin::*;

pub mod binance;
pub use binance::*;

pub mod transaction;
pub use transaction::*;

pub mod user;
pub use user::*;

/// Used to skip serialization if the value is default e.g. 0 or ""
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

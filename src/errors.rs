//! Custom error types to be shared across lambdas

/// Used as a generic error type to be serialized and returned to caller
pub type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

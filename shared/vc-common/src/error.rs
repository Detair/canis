//! Common Error Types

use thiserror::Error;

/// Common error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Authentication error
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Authorization error
    #[error("Not authorized: {0}")]
    Forbidden(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation failed: {0}")]
    Validation(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimited,

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Common result type.
pub type Result<T> = std::result::Result<T, Error>;

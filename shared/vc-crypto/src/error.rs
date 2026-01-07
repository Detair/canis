//! Crypto Error Types

use thiserror::Error;

/// Cryptography error type.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Invalid key
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Signature verification failed
    #[error("Signature verification failed")]
    SignatureInvalid,

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// vodozemac error
    #[error("Crypto error: {0}")]
    Vodozemac(String),
}

/// Crypto result type.
pub type Result<T> = std::result::Result<T, CryptoError>;

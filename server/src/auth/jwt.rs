//! JWT Token Generation and Validation

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT claims for access tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Token type
    pub typ: TokenType,
}

/// Token type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

/// Generate a new access token.
pub fn generate_access_token(
    _user_id: &str,
    _secret: &str,
    _expiry_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    // TODO: Implement
    todo!()
}

/// Generate a new refresh token.
pub fn generate_refresh_token(
    _user_id: &str,
    _secret: &str,
    _expiry_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    // TODO: Implement
    todo!()
}

/// Validate and decode a token.
pub fn validate_token(_token: &str, _secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // TODO: Implement
    todo!()
}

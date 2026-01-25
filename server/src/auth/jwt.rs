//! JWT Token Generation and Validation
//!
//! Uses RS256 (RSA-SHA256) for asymmetric token signing/verification.
//! This allows separate signing (private key) and verification (public key),
//! which is more secure than symmetric HS256 and supports distributed architectures.

use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::{AuthError, AuthResult};

/// JWT claims for access and refresh tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID as UUID string).
    pub sub: String,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// Issued at (Unix timestamp).
    pub iat: i64,
    /// Token type (access or refresh).
    pub typ: TokenType,
    /// JWT ID for refresh token revocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

/// Token type discriminator.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    /// Short-lived access token.
    Access,
    /// Long-lived refresh token.
    Refresh,
}

/// Token pair returned after successful authentication.
#[derive(Debug)]
pub struct TokenPair {
    /// Access token (short-lived).
    pub access_token: String,
    /// Refresh token (long-lived).
    pub refresh_token: String,
    /// Access token expiry in seconds.
    pub access_expires_in: i64,
    /// Refresh token ID for session tracking.
    pub refresh_token_id: Uuid,
}

/// Decode a base64-encoded PEM key.
fn decode_pem_key(base64_key: &str) -> AuthResult<Vec<u8>> {
    STANDARD
        .decode(base64_key)
        .map_err(|_| AuthError::Internal("Invalid base64 in JWT key".to_string()))
}

/// Generate both access and refresh tokens.
///
/// # Arguments
/// * `user_id` - The user's UUID
/// * `private_key` - RSA private key (PEM format, base64-encoded)
/// * `access_expiry_seconds` - Access token validity (typically 900 = 15 min)
/// * `refresh_expiry_seconds` - Refresh token validity (typically 604800 = 7 days)
pub fn generate_token_pair(
    user_id: Uuid,
    private_key: &str,
    access_expiry_seconds: i64,
    refresh_expiry_seconds: i64,
) -> AuthResult<TokenPair> {
    let now = Utc::now();
    let refresh_token_id = Uuid::now_v7();

    // Decode the private key from base64-encoded PEM
    let key_bytes = decode_pem_key(private_key)?;
    let encoding_key = EncodingKey::from_rsa_pem(&key_bytes)
        .map_err(|e| AuthError::Internal(format!("Invalid RSA private key: {e}")))?;

    // Access token
    let access_claims = Claims {
        sub: user_id.to_string(),
        exp: (now + Duration::seconds(access_expiry_seconds)).timestamp(),
        iat: now.timestamp(),
        typ: TokenType::Access,
        jti: None,
    };

    let access_token = encode(
        &Header::new(Algorithm::RS256),
        &access_claims,
        &encoding_key,
    )?;

    // Refresh token (includes jti for revocation tracking)
    let refresh_claims = Claims {
        sub: user_id.to_string(),
        exp: (now + Duration::seconds(refresh_expiry_seconds)).timestamp(),
        iat: now.timestamp(),
        typ: TokenType::Refresh,
        jti: Some(refresh_token_id.to_string()),
    };

    let refresh_token = encode(
        &Header::new(Algorithm::RS256),
        &refresh_claims,
        &encoding_key,
    )?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        access_expires_in: access_expiry_seconds,
        refresh_token_id,
    })
}

/// Validate and decode an access token.
///
/// Returns an error if the token is invalid, expired, or is a refresh token.
pub fn validate_access_token(token: &str, public_key: &str) -> AuthResult<Claims> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.leeway = 0;

    // Decode the public key from base64-encoded PEM
    let key_bytes = decode_pem_key(public_key)?;
    let decoding_key = DecodingKey::from_rsa_pem(&key_bytes)
        .map_err(|e| AuthError::Internal(format!("Invalid RSA public key: {e}")))?;

    let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| match e.kind()
    {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::InvalidToken,
    })?;

    // Ensure it's an access token
    if token_data.claims.typ != TokenType::Access {
        return Err(AuthError::InvalidToken);
    }

    Ok(token_data.claims)
}

/// Validate and decode a refresh token.
///
/// Returns an error if the token is invalid, expired, or is an access token.
pub fn validate_refresh_token(token: &str, public_key: &str) -> AuthResult<Claims> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.leeway = 0;

    // Decode the public key from base64-encoded PEM
    let key_bytes = decode_pem_key(public_key)?;
    let decoding_key = DecodingKey::from_rsa_pem(&key_bytes)
        .map_err(|e| AuthError::Internal(format!("Invalid RSA public key: {e}")))?;

    let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| match e.kind()
    {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::InvalidToken,
    })?;

    // Ensure it's a refresh token
    if token_data.claims.typ != TokenType::Refresh {
        return Err(AuthError::InvalidToken);
    }

    // Refresh tokens MUST have a jti
    if token_data.claims.jti.is_none() {
        return Err(AuthError::InvalidToken);
    }

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test RSA key pair (2048-bit) - same as in config.rs for test consistency
    const TEST_PRIVATE_KEY: &str = "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0tCk1JSUV2Z0lCQURBTkJna3Foa2lHOXcwQkFRRUZBQVNDQktnd2dnU2tBZ0VBQW9JQkFRQ3J3MVJCSFVLSy9TUXoKaWpGYTJBQkg2bjZHV1JsSGxoMWFIOExGbkxFNEQ4ZStONWowRkdPeTYzTkdnVTBpcFY3eXViZHhodzRSdTdCdgptQ2dWN1N4T294VGQ4bEcrczNTOC9XeFpFTHU0MWh2ZnJGNVVsRTBnTml1SnV5TG1IeDdMWmVGOTFBd2FhMjg1CitPWlhvdFZzN01CRm15a214clA1TEVnQWZuc2NlUCt0bldlSVBQVEZMb0V1MkI4RXRLd2tQSVFUUmUzTmpRbXQKS1NkcnF5SFRUUVlQWnppWktQS1RzeWFzUFZERHJuckYyZWc1S0dKRzBXZjY1OWlaT2cxMCtXTDQwelBsdEFONApGSkhHOVVDWjlvSEo1RVhlRExqT0VlaE5kaUZPUlp4Kzh6Y0dnZGJQQzVaOXJ2SUl2MHhzL3JMamxSN2h4QXpnCkpMbFEycXJ0QWdNQkFBRUNnZ0VBQVd6bHl3T0tIckJ2Q3gwQm5Ed2lKbmFoMlRoS1dDcEQvMEliUlpoRTBhdDUKMnNsSURPbGdFbEhYWW1yZmNiOTNFMVRjdHlsR1NQbmpab2FRM1Fud3pDa0trOE1NeVFDb0U5SXprTmNZSUNoYgpYODNXaGpyRm02SmFIVTh2QVB4RXZVckNnclE1RmtSOEk1M1RkMDNoTmwzeG1jQi9MNTlHa2NyUXp5WTlqYzNXCmhXVkRaOXVSU0UvdVk4Znp2SmdRM1JiUGliZkgwekg3QUhBMkJ2aE40NTVOdW9kVkwyYkphMXpUZ2NaR0E4QVUKb21haVArYytHNTJUME5tL0FJUUl4Tnc1UWtIa3F4cVlHaFpwRGhuWHp6OTlFcW1IWVZYaFVBeVBpMXJZWVRTLwplYk4yM2wzaTMxeTBhREhnVmI5RVh2UGJmbFRvY2FlSW40VGtVSHhza1FLQmdRRHVqbWsybTJCN0pLaC9xVE9ICm9jcTA3REFXVlRtYU92ZjlIUHYzRXlpY2ppVmtlQnJRVzZWM3JQQzRadHMyY3lidFdhRHFMWW5SNy9aZHphYmIKWm5vV24vbFVOS2szYUdaYWVkbmphRlJTQjFXZ1lQWURLL1h4bHdOVDlBdVdXNEdVVXd4b0M5SUp4OGpkbVpTaQo3cFRkVCtaZDUrV21ZWlhTWW5QUjZoS0RVUUtCZ1FDNFVwbDVVWExUblhmR3l2TmpMbkhGbmZRTWh4TGlFKytBCm10bXhqR2dFbnI5Q0hidzdCVHpuNUVCSitxVFl0SFU1RWUzNWFaR1AwVktVWng4eGp4aWpHOUd6eFlTRDBjb2sKNHcyWWtXSGkwOTFCTk42YVFrM090dC9vdzNTOFVvaTNPSGdZQ0JtQmtteFlRcEdYNlpIRHJDS2ZGVUV4K0R3ZApUdkNsUmFydTNRS0JnUURkUDN0WlIvWE5nQXcraWtEZWRERzZacXVhcXVSSHBKVkhUVkJxc0h3ajVybkxXcEVUCjJVdTNtTStSVnVQTXRqUE9RaWc1eUk1Z0JQd3J0NFlmU2dYRllnMHVDY0UvUURaZGgxR0wxY0VPYXZzQlNhd2cKK082YmFBR1FKWEZ4dStDTUhoSU5sWmp4dFRjWVAwNVpab2p1VVNKSXljQjE5VitzeGQ3Qk95UjhZUUtCZ0RocQpkN1VOTytNUFVHalZGM2VrOEllMjE4cTUwUXJIWlVmc25YTGRjYnp3UmNQYnpCQVlnMUxLcHU2OXU1VGtidmlmCngwSE9rUkgrMUpLOW1XdVd5OGlvckIrazlmRk8xZHRDYjVmaDc1NzRqOEQwaUttWVg2NUVoUFgrVlEyTENYTmkKNGtjZ3U0WFFKajlCYU1TaFpjOEpNYk9WVXRZVGozcTgvYVRvVlBBMUFvR0JBTFByZTBOTWJqaHludzVqR1VtSQp6L0VZcWNOc2NtZ3JDMDdkMTRtNGxVWVVyblcwQ2FyOTIwbFdZcStQc3k1T2d4dEs4WjE4WndMcXVkMmtaNWxRCm1HOE8rcmZYeXZldHNkRWlQYXZNVnBGRmE1OG5ERG16dWtKb0tuM3RZY0JsT2d4b29ZUjlyb2hnd3VuZDlXZXEKUGNUMFVKRjVtQXFQUUw5YkRJaGZaSXN4Ci0tLS0tRU5EIFBSSVZBVEUgS0VZLS0tLS0K";
    const TEST_PUBLIC_KEY: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUFxOE5VUVIxQ2l2MGtNNG94V3RnQQpSK3AraGxrWlI1WWRXaC9DeFp5eE9BL0h2amVZOUJSanN1dHpSb0ZOSXFWZThybTNjWWNPRWJ1d2I1Z29GZTBzClRxTVUzZkpSdnJOMHZQMXNXUkM3dU5ZYjM2eGVWSlJOSURZcmlic2k1aDhleTJYaGZkUU1HbXR2T2ZqbVY2TFYKYk96QVJac3BKc2F6K1N4SUFINTdISGovcloxbmlEejB4UzZCTHRnZkJMU3NKRHlFRTBYdHpZMEpyU2tuYTZzaAowMDBHRDJjNG1TanlrN01tckQxUXc2NTZ4ZG5vT1NoaVJ0Rm4rdWZZbVRvTmRQbGkrTk16NWJRRGVCU1J4dlZBCm1mYUJ5ZVJGM2d5NHpoSG9UWFloVGtXY2Z2TTNCb0hXend1V2ZhN3lDTDlNYlA2eTQ1VWU0Y1FNNENTNVVOcXEKN1FJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==";

    // A different RSA public key for testing validation failure (mismatched key pair)
    const WRONG_PUBLIC_KEY: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUF5Wm13eGQzVFRRQlozbGZNTmZoYgorSm1jYlAwQlhXaFoxZXhESDFsdWx6aytEZFpBNXJPWG10cW5nWmJPdlZCY2dRWE14em5wZDYyb04xWWh3MXBoCk9CdFlPTDloVys4VEkveHhmbWdGVlg0NTFWZ2tMRUZMWEw5N1NCUC82WlZ5MVY3Zy9DYWlhb2twTmJUNkRxUXkKa3BUSVVFYlBaNDhhaHk4cEpaRzk0TFdYWGVhMStFeHNxZ3dielVhamFQYTBjb0F0Y1A5TjFiY2xmeitrV2MyQQphYXFuaHEzbnZEelVzRk42VUkybEJSYkNFZ1JYbS9VMDRSMVBOSzhsWElYd1htckdORHpBbksyNUxIUHpZa0VaClRLcE5GckorSnJJdjF1Wkh5T2JDY09EYUMxeEd0cGpSQ0NFTEphUXBYbGltelpJZGZtRWxpUnFuODI3bjk1YXoKU3dJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==";

    #[test]
    fn test_generate_token_pair() {
        let user_id = Uuid::now_v7();

        let tokens = generate_token_pair(user_id, TEST_PRIVATE_KEY, 900, 604800).unwrap();

        assert!(!tokens.access_token.is_empty());
        assert!(!tokens.refresh_token.is_empty());
        assert_eq!(tokens.access_expires_in, 900);
    }

    #[test]
    fn test_validate_access_token() {
        let user_id = Uuid::now_v7();

        let tokens = generate_token_pair(user_id, TEST_PRIVATE_KEY, 900, 604800).unwrap();
        let claims = validate_access_token(&tokens.access_token, TEST_PUBLIC_KEY).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.typ, TokenType::Access);
    }

    #[test]
    fn test_validate_refresh_token() {
        let user_id = Uuid::now_v7();

        let tokens = generate_token_pair(user_id, TEST_PRIVATE_KEY, 900, 604800).unwrap();
        let claims = validate_refresh_token(&tokens.refresh_token, TEST_PUBLIC_KEY).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.typ, TokenType::Refresh);
        assert!(claims.jti.is_some());
    }

    #[test]
    fn test_access_token_rejects_refresh_token() {
        let user_id = Uuid::now_v7();

        let tokens = generate_token_pair(user_id, TEST_PRIVATE_KEY, 900, 604800).unwrap();
        let result = validate_access_token(&tokens.refresh_token, TEST_PUBLIC_KEY);

        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token_rejects_access_token() {
        let user_id = Uuid::now_v7();

        let tokens = generate_token_pair(user_id, TEST_PRIVATE_KEY, 900, 604800).unwrap();
        let result = validate_refresh_token(&tokens.access_token, TEST_PUBLIC_KEY);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_secret_fails() {
        let user_id = Uuid::now_v7();

        let tokens = generate_token_pair(user_id, TEST_PRIVATE_KEY, 900, 604800).unwrap();
        let result = validate_access_token(&tokens.access_token, WRONG_PUBLIC_KEY);

        assert!(result.is_err());
    }
}

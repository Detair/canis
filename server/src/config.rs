//! Server Configuration
//!
//! Loads configuration from environment variables.

use anyhow::{Context, Result};
use std::env;

/// Server configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Server bind address (e.g., "0.0.0.0:8080")
    pub bind_address: String,

    /// `PostgreSQL` connection URL
    pub database_url: String,

    /// Valkey/Redis connection URL (uses redis:// protocol)
    pub redis_url: String,

    /// JWT private key (PEM format, base64 encoded) for signing tokens
    pub jwt_private_key: String,

    /// JWT public key (PEM format, base64 encoded) for verifying tokens
    pub jwt_public_key: String,

    /// JWT access token expiry in seconds (default: 900 = 15 min)
    pub jwt_access_expiry: i64,

    /// JWT refresh token expiry in seconds (default: 604800 = 7 days)
    pub jwt_refresh_expiry: i64,

    /// S3-compatible storage endpoint
    pub s3_endpoint: Option<String>,

    /// S3 bucket name
    pub s3_bucket: String,

    /// S3 presigned URL expiry in seconds (default: 3600 = 1 hour)
    pub s3_presign_expiry: i64,

    /// Allowed MIME types for file uploads (comma-separated)
    pub allowed_mime_types: Option<Vec<String>>,

    /// OIDC issuer URL (optional)
    pub oidc_issuer_url: Option<String>,

    /// OIDC client ID (optional)
    pub oidc_client_id: Option<String>,

    /// OIDC client secret (optional)
    pub oidc_client_secret: Option<String>,

    /// Maximum file upload size in bytes (default: 50MB)
    pub max_upload_size: usize,

    /// WebRTC STUN server
    pub stun_server: String,

    /// WebRTC TURN server (optional)
    pub turn_server: Option<String>,

    /// WebRTC TURN username (optional)
    pub turn_username: Option<String>,

    /// WebRTC TURN credential (optional)
    pub turn_credential: Option<String>,

    /// MFA secret encryption key (32-byte hex string)
    pub mfa_encryption_key: Option<String>,

    /// Whether E2EE setup is required before using the app (default: false)
    pub require_e2ee_setup: bool,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_private_key: env::var("JWT_PRIVATE_KEY").context("JWT_PRIVATE_KEY must be set (base64-encoded PEM)")?,
            jwt_public_key: env::var("JWT_PUBLIC_KEY").context("JWT_PUBLIC_KEY must be set (base64-encoded PEM)")?,
            jwt_access_expiry: env::var("JWT_ACCESS_EXPIRY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(900),
            jwt_refresh_expiry: env::var("JWT_REFRESH_EXPIRY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(604800),
            s3_endpoint: env::var("S3_ENDPOINT").ok(),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "voicechat".into()),
            s3_presign_expiry: env::var("S3_PRESIGN_EXPIRY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3600), // 1 hour
            allowed_mime_types: env::var("ALLOWED_MIME_TYPES").ok().map(|s| {
                s.split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect()
            }),
            oidc_issuer_url: env::var("OIDC_ISSUER_URL").ok(),
            oidc_client_id: env::var("OIDC_CLIENT_ID").ok(),
            oidc_client_secret: env::var("OIDC_CLIENT_SECRET").ok(),
            max_upload_size: env::var("MAX_UPLOAD_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50 * 1024 * 1024), // 50MB
            stun_server: env::var("STUN_SERVER")
                .unwrap_or_else(|_| "stun:stun.l.google.com:19302".into()),
            turn_server: env::var("TURN_SERVER").ok(),
            turn_username: env::var("TURN_USERNAME").ok(),
            turn_credential: env::var("TURN_CREDENTIAL").ok(),
            mfa_encryption_key: env::var("MFA_ENCRYPTION_KEY").ok(),
            require_e2ee_setup: env::var("REQUIRE_E2EE_SETUP")
                .ok()
                .map(|v| v.to_lowercase() == "true" || v == "1")
                .unwrap_or(false),
        })
    }

    /// Check if OIDC is configured.
    #[must_use]
    pub const fn has_oidc(&self) -> bool {
        self.oidc_issuer_url.is_some()
            && self.oidc_client_id.is_some()
            && self.oidc_client_secret.is_some()
    }

    /// Check if TURN is configured.
    #[must_use]
    pub const fn has_turn(&self) -> bool {
        self.turn_server.is_some()
    }

    /// Create a default configuration for testing.
    ///
    /// Uses Docker test containers:
    /// - `PostgreSQL`: `docker run -d --name canis-test-postgres -e POSTGRESQL_USERNAME=test -e POSTGRESQL_PASSWORD=test -e POSTGRESQL_DATABASE=test -p 5434:5432 bitnami/postgresql:latest`
    /// - Redis: `docker run -d --name canis-test-redis -e ALLOW_EMPTY_PASSWORD=yes -p 6380:6379 bitnami/redis:latest`
    ///
    /// Run migrations: `DATABASE_URL="postgresql://test:test@localhost:5434/test" sqlx migrate run --source server/migrations`
    #[must_use]
    pub fn default_for_test() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".into(),
            // Uses dev database - sqlx::test creates isolated DBs automatically
            database_url: "postgresql://voicechat:voicechat_dev@localhost:5433/voicechat".into(),
            // Uses dev Redis
            redis_url: "redis://localhost:6379".into(),
            // Test RSA key pair (2048-bit, generated for testing only)
            jwt_private_key: TEST_JWT_PRIVATE_KEY.into(),
            jwt_public_key: TEST_JWT_PUBLIC_KEY.into(),
            jwt_access_expiry: 900,
            jwt_refresh_expiry: 604800,
            s3_endpoint: None,
            s3_bucket: "test-bucket".into(),
            s3_presign_expiry: 3600,
            allowed_mime_types: None,
            max_upload_size: 50 * 1024 * 1024,
            oidc_issuer_url: None,
            oidc_client_id: None,
            oidc_client_secret: None,
            stun_server: "stun:stun.l.google.com:19302".into(),
            turn_server: None,
            turn_username: None,
            turn_credential: None,
            mfa_encryption_key: None,
            require_e2ee_setup: false,
        }
    }
}

// Test RSA key pair (2048-bit) - DO NOT USE IN PRODUCTION
// Generated with: openssl genrsa -out private.pem 2048 && openssl rsa -in private.pem -pubout -out public.pem
// Then base64-encoded for storage in environment variables

/// Test private key (base64-encoded PEM)
const TEST_JWT_PRIVATE_KEY: &str = "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0tCk1JSUV2Z0lCQURBTkJna3Foa2lHOXcwQkFRRUZBQVNDQktnd2dnU2tBZ0VBQW9JQkFRQ3J3MVJCSFVLSy9TUXoKaWpGYTJBQkg2bjZHV1JsSGxoMWFIOExGbkxFNEQ4ZStONWowRkdPeTYzTkdnVTBpcFY3eXViZHhodzRSdTdCdgptQ2dWN1N4T294VGQ4bEcrczNTOC9XeFpFTHU0MWh2ZnJGNVVsRTBnTml1SnV5TG1IeDdMWmVGOTFBd2FhMjg1CitPWlhvdFZzN01CRm15a214clA1TEVnQWZuc2NlUCt0bldlSVBQVEZMb0V1MkI4RXRLd2tQSVFUUmUzTmpRbXQKS1NkcnF5SFRUUVlQWnppWktQS1RzeWFzUFZERHJuckYyZWc1S0dKRzBXZjY1OWlaT2cxMCtXTDQwelBsdEFONApGSkhHOVVDWjlvSEo1RVhlRExqT0VlaE5kaUZPUlp4Kzh6Y0dnZGJQQzVaOXJ2SUl2MHhzL3JMamxSN2h4QXpnCkpMbFEycXJ0QWdNQkFBRUNnZ0VBQVd6bHl3T0tIckJ2Q3gwQm5Ed2lKbmFoMlRoS1dDcEQvMEliUlpoRTBhdDUKMnNsSURPbGdFbEhYWW1yZmNiOTNFMVRjdHlsR1NQbmpab2FRM1Fud3pDa0trOE1NeVFDb0U5SXprTmNZSUNoYgpYODNXaGpyRm02SmFIVTh2QVB4RXZVckNnclE1RmtSOEk1M1RkMDNoTmwzeG1jQi9MNTlHa2NyUXp5WTlqYzNXCmhXVkRaOXVSU0UvdVk4Znp2SmdRM1JiUGliZkgwekg3QUhBMkJ2aE40NTVOdW9kVkwyYkphMXpUZ2NaR0E4QVUKb21haVArYytHNTJUME5tL0FJUUl4Tnc1UWtIa3F4cVlHaFpwRGhuWHp6OTlFcW1IWVZYaFVBeVBpMXJZWVRTLwplYk4yM2wzaTMxeTBhREhnVmI5RVh2UGJmbFRvY2FlSW40VGtVSHhza1FLQmdRRHVqbWsybTJCN0pLaC9xVE9ICm9jcTA3REFXVlRtYU92ZjlIUHYzRXlpY2ppVmtlQnJRVzZWM3JQQzRadHMyY3lidFdhRHFMWW5SNy9aZHphYmIKWm5vV24vbFVOS2szYUdaYWVkbmphRlJTQjFXZ1lQWURLL1h4bHdOVDlBdVdXNEdVVXd4b0M5SUp4OGpkbVpTaQo3cFRkVCtaZDUrV21ZWlhTWW5QUjZoS0RVUUtCZ1FDNFVwbDVVWExUblhmR3l2TmpMbkhGbmZRTWh4TGlFKytBCm10bXhqR2dFbnI5Q0hidzdCVHpuNUVCSitxVFl0SFU1RWUzNWFaR1AwVktVWng4eGp4aWpHOUd6eFlTRDBjb2sKNHcyWWtXSGkwOTFCTk42YVFrM090dC9vdzNTOFVvaTNPSGdZQ0JtQmtteFlRcEdYNlpIRHJDS2ZGVUV4K0R3ZApUdkNsUmFydTNRS0JnUURkUDN0WlIvWE5nQXcraWtEZWRERzZacXVhcXVSSHBKVkhUVkJxc0h3ajVybkxXcEVUCjJVdTNtTStSVnVQTXRqUE9RaWc1eUk1Z0JQd3J0NFlmU2dYRllnMHVDY0UvUURaZGgxR0wxY0VPYXZzQlNhd2cKK082YmFBR1FKWEZ4dStDTUhoSU5sWmp4dFRjWVAwNVpab2p1VVNKSXljQjE5VitzeGQ3Qk95UjhZUUtCZ0RocQpkN1VOTytNUFVHalZGM2VrOEllMjE4cTUwUXJIWlVmc25YTGRjYnp3UmNQYnpCQVlnMUxLcHU2OXU1VGtidmlmCngwSE9rUkgrMUpLOW1XdVd5OGlvckIrazlmRk8xZHRDYjVmaDc1NzRqOEQwaUttWVg2NUVoUFgrVlEyTENYTmkKNGtjZ3U0WFFKajlCYU1TaFpjOEpNYk9WVXRZVGozcTgvYVRvVlBBMUFvR0JBTFByZTBOTWJqaHludzVqR1VtSQp6L0VZcWNOc2NtZ3JDMDdkMTRtNGxVWVVyblcwQ2FyOTIwbFdZcStQc3k1T2d4dEs4WjE4WndMcXVkMmtaNWxRCm1HOE8rcmZYeXZldHNkRWlQYXZNVnBGRmE1OG5ERG16dWtKb0tuM3RZY0JsT2d4b29ZUjlyb2hnd3VuZDlXZXEKUGNUMFVKRjVtQXFQUUw5YkRJaGZaSXN4Ci0tLS0tRU5EIFBSSVZBVEUgS0VZLS0tLS0K";

/// Test public key (base64-encoded PEM)
const TEST_JWT_PUBLIC_KEY: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUFxOE5VUVIxQ2l2MGtNNG94V3RnQQpSK3AraGxrWlI1WWRXaC9DeFp5eE9BL0h2amVZOUJSanN1dHpSb0ZOSXFWZThybTNjWWNPRWJ1d2I1Z29GZTBzClRxTVUzZkpSdnJOMHZQMXNXUkM3dU5ZYjM2eGVWSlJOSURZcmlic2k1aDhleTJYaGZkUU1HbXR2T2ZqbVY2TFYKYk96QVJac3BKc2F6K1N4SUFINTdISGovcloxbmlEejB4UzZCTHRnZkJMU3NKRHlFRTBYdHpZMEpyU2tuYTZzaAowMDBHRDJjNG1TanlrN01tckQxUXc2NTZ4ZG5vT1NoaVJ0Rm4rdWZZbVRvTmRQbGkrTk16NWJRRGVCU1J4dlZBCm1mYUJ5ZVJGM2d5NHpoSG9UWFloVGtXY2Z2TTNCb0hXend1V2ZhN3lDTDlNYlA2eTQ1VWU0Y1FNNENTNVVOcXEKN1FJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==";

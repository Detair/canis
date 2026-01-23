//! Client-side Cryptography
//!
//! Local storage and management of cryptographic keys for E2EE messaging.

pub mod manager;
pub mod store;

pub use manager::{ClaimedPrekey, CryptoManager, PrekeyForUpload, PrekeyInfo};

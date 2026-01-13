//! Permission system types and utilities.
//!
//! Two-tier permission model:
//! - System permissions: Platform-level admin actions
//! - Guild permissions: Per-guild role-based access control

pub mod guild;
pub mod models;
pub mod system;

pub use guild::GuildPermissions;
pub use models::*;
pub use system::SystemPermission;

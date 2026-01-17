//! Rate limiting constants.

/// IPv6 prefix segments for rate limiting (uses /64)
pub const IPV6_PREFIX_SEGMENTS: usize = 4;

/// Lua script return codes
pub const SCRIPT_ALLOWED: i64 = 1;

// Note: The following constants are defined for documentation/future use:
// - SCRIPT_DENIED (0): Return code when rate limit exceeded
// - TTL_NO_EXPIRY (-1): Redis TTL when key has no expiry
// - TTL_KEY_NOT_FOUND (-2): Redis TTL when key doesn't exist

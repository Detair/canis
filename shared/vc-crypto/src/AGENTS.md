<!-- Parent: ../AGENTS.md -->
# vc-crypto Source

## Purpose

Core implementation of end-to-end encryption primitives using vodozemac. Contains the actual cryptographic operations for Olm (1:1 messaging) and Megolm (group messaging) protocols.

## Key Files

| File | Purpose |
|------|---------|
| `lib.rs` | Public API, re-exports vodozemac types, module organization |
| `error.rs` | `CryptoError` enum with thiserror for typed error handling |
| `olm.rs` | Olm account and session wrappers for 1:1 Double Ratchet encryption |
| `megolm.rs` | Megolm inbound/outbound sessions for efficient group encryption |

## For AI Agents

### Critical Security Constraints

**NEVER:**
- Log private keys or decrypted content
- Serialize unencrypted sessions to JSON
- Implement custom crypto algorithms
- Mock crypto operations in tests

**ALWAYS:**
- Use `#[derive(Zeroize, ZeroizeOnDrop)]` for key material
- Use vodozemac primitives only
- Test against real vodozemac instances
- Verify encrypt-decrypt round-trips

### Module Responsibilities

**`lib.rs`:**
- Re-export public types from vodozemac
- Define `Result<T>` type alias with `CryptoError`
- Module declarations

**`error.rs`:**
- `CryptoError` variants: `VodozemacError`, `InvalidMessage`, `SessionError`
- Convert underlying library errors

**`olm.rs`:**
- `OlmAccount` - Identity keys, one-time keys generation
- `OlmSession` - Encrypt/decrypt for 1:1 communication
- Session creation (inbound/outbound)

**`megolm.rs`:**
- `OutboundGroupSession` - Encrypt messages to group
- `InboundGroupSession` - Decrypt received group messages
- Session key export/import

### Common Patterns

```rust
// Olm session creation
let alice = OlmAccount::new();
let bob_session = bob.create_outbound_session(alice_keys.curve25519)?;

// Megolm group encryption
let mut outbound = OutboundGroupSession::new();
let ciphertext = outbound.encrypt("Hello group")?;
```

### Testing Guidelines

- [ ] Encrypt-decrypt round-trip works
- [ ] Multiple messages decrypt in order
- [ ] Session state persists across serialization
- [ ] Invalid ciphertext returns error (not panic)
- [ ] Keys are zeroized on drop

## Dependencies

- `vodozemac` - Core Olm/Megolm implementation (Apache-2.0)
- `serde` - Serialization for encrypted messages
- `zeroize` - Secure memory clearing
- `thiserror` - Error type derivation

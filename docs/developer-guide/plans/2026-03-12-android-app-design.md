# Android App Design — Native Kotlin + Jetpack Compose

**Date:** 2026-03-12
**Status:** Approved
**Supersedes:** `2026-02-01-mobile-app-design.md` (for Android scope)

## Overview

Native Android app for Kaiku v1. Jetpack Compose + Kotlin, pure Kotlin networking, no UniFFI/Rust core. JNI vodozemac wrapper for E2EE in milestone 2. Two milestones: core loop (text + voice), then full experience (E2EE, push, DMs, files). Android-only — no iOS (no test device). This app will be replaced by a native v2 client; the goal is a usable v1 mobile client and native Android development experience.

---

## 1. Project Structure

```
kaiku/
  mobile/
    android/                          # Android app (Jetpack Compose)
      app/
        build.gradle.kts
        src/main/
          java/io/wolftown/kaiku/
            KaikuApplication.kt
            di/                       # Hilt DI modules
            data/
              api/                    # Ktor HTTP client, API DTOs
              ws/                     # OkHttp WebSocket client
              repository/             # AuthRepo, ChatRepo, GuildRepo, VoiceRepo
              local/                  # EncryptedSharedPreferences, minimal cache
            domain/model/             # Domain models (Guild, Channel, Message, User)
            ui/
              auth/                   # Login, Register, QR scan
              home/                   # Guild list sidebar
              channel/                # Text channel, message list
              voice/                  # Voice channel UI, controls
              dm/                     # DM conversations
              friends/                # Friends list
              settings/               # Settings, session management
              shared/                 # Shared composables
            service/                  # VoiceCallService (foreground), PushService
            util/                     # QR helpers, audio routing
      gradle/
      settings.gradle.kts
    vc-vodozemac-jni/                 # Milestone 2: thin JNI wrapper for vodozemac
      Cargo.toml
      src/lib.rs
      build.rs
```

- Standard Android project with Hilt DI, MVVM pattern
- `vc-vodozemac-jni` is a separate Rust crate added in milestone 2 — only wraps vodozemac crypto primitives (Olm account, sessions, encrypt/decrypt), not networking
- No `vc-mobile-core` — networking is all Kotlin

---

## 2. Tech Stack & Dependencies

### Android

- **Min SDK:** 26 (Android 8.0) — covers ~95% of active devices
- **Target SDK:** 35
- **Language:** Kotlin 2.x
- **UI:** Jetpack Compose + Material 3
- **DI:** Hilt
- **Navigation:** Navigation Compose
- **Networking:** Ktor (HTTP client, REST API) + OkHttp (WebSocket)
- **Serialization:** kotlinx.serialization (snake_case wire format matches server serde defaults)
- **Voice:** stream-webrtc-android
- **Push:** Firebase Cloud Messaging (FCM)
- **Auth storage:** AndroidX Security Crypto (EncryptedSharedPreferences)
- **Image loading:** Coil (Compose-native)
- **Camera/QR:** CameraX + ML Kit Barcode Scanning (QR Challenge-Response)
- **Build:** Gradle Kotlin DSL

### Milestone 2 Additions

- `vc-vodozemac-jni` Rust crate compiled via `cargo-ndk` for arm64-v8a, armeabi-v7a, x86_64
- vodozemac 0.9, jni crate for Rust-side bindings

### Not Used (vs. Original Plan)

- No UniFFI, no reqwest, no tokio-tungstenite, no rusqlite, no mdns-sd
- No Room database (minimal caching only)
- No Google Play Services FIDO2
- No qrcode-kotlin (only QR scanning, not generation — mobile scans the desktop QR)

---

## 3. Milestone 1 — Core Loop

**Goal:** A usable Android app for participating in a gaming session.

### Features

1. **Login** — username/password + existing TOTP MFA + OIDC (via Custom Tab with `canis://oidc/callback`)
2. **Guild list + channel navigation** — sidebar with guilds, channel list per guild, unread indicators
3. **Text messaging** — send, receive, edit, delete in real-time via WebSocket. Markdown rendering. Reactions.
4. **Voice chat** — join/leave voice channels, mic mute/unmute, speaker list with voice activity indicators, audio routing (speaker/earpiece/bluetooth). Foreground service for background audio.
5. **Screen share viewing** — watch screen shares from desktop users (receive-only, no sharing from phone)

### Architecture

```
┌─────────────────────────────────────┐
│         Jetpack Compose UI          │
│   (Screens, ViewModels, State)      │
├─────────────────────────────────────┤
│         Repository Layer            │
│  AuthRepo  ChatRepo  VoiceRepo     │
│  GuildRepo                          │
├──────────────┬──────────────────────┤
│  Ktor HTTP   │  OkHttp WebSocket   │
│  (REST API)  │  (real-time events)  │
├──────────────┴──────────────────────┤
│  stream-webrtc-android              │
│  (voice + screen share viewing)     │
└─────────────────────────────────────┘
         │              │
    Kaiku REST API   Kaiku WebSocket
         │              │
    ┌────▼──────────────▼────┐
    │     Kaiku Server       │
    └────────────────────────┘
```

### Server Changes

None. The existing REST API, WebSocket protocol, and voice signaling work as-is.

### WebSocket Protocol

Identical to desktop — `Sec-WebSocket-Protocol: access_token.<jwt>`, same JSON event types.

### Voice Flow

Same signaling as desktop (VoiceJoin → server sends VoiceOffer → VoiceAnswer → ICE candidates). `stream-webrtc-android` handles Opus codec, echo cancellation, noise suppression natively. `VoiceCallService` as foreground service with `FOREGROUND_SERVICE_TYPE_MICROPHONE` for background audio.

---

## 4. Milestone 2 — Full Experience

**Goal:** Feature-complete v1 Android client.

### Features

1. **E2EE for text messages** — JNI vodozemac wrapper. Olm for DMs, Megolm for group channels. Same protocol as desktop client. Device identity key registration via existing `/api/crypto/` endpoints.
2. **Push notifications** — FCM integration. Requires new server module (`server/src/push/`): token registration endpoints, dispatch logic that sends push when user has no active WebSocket.
3. **DMs / Friends** — friend list, add/remove/block, DM conversations. All existing server endpoints.
4. **File uploads / image viewing** — upload via presigned S3 URLs (already implemented on server), image preview with Coil, file download.
5. **Member list / user profiles** — guild member list, user profiles with avatar, status, roles.
6. **QR Challenge-Response** — mobile scans QR displayed on desktop, signs nonce with device Ed25519 key, approves login.

### Server Changes

| Change | Scope |
|---|---|
| Push notification module | New `server/src/push/` — handlers, FCM dispatcher, token management |
| Push tokens table | New migration: `push_tokens` table |
| QR Challenge-Response | New `server/src/auth/qr_challenge.rs` — 3 endpoints |
| QR login challenges table | New migration: `qr_login_challenges` table |
| Rate limit categories | Add `QrAuth`, `PushRegister` to rate limiter |

### New Server Endpoints

#### QR Challenge-Response

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/auth/qr/challenge` | No | Desktop creates QR challenge. Returns `{nonce, poll_token, qr_data, expires_at}`. |
| GET | `/api/auth/qr/challenge/{nonce}` | Poll token | Desktop polls for approval. Rate: 1 req/2s max. |
| POST | `/api/auth/qr/approve` | Yes | Mobile approves with device signature. |

#### Push Notifications

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/push/register` | Yes | Register FCM token. |
| POST | `/api/push/refresh` | Yes | Update token after FCM refresh. |
| DELETE | `/api/push/register` | Yes | Unregister token. |

### Database Migrations

#### QR Login Challenges

```sql
CREATE TABLE qr_login_challenges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_nonce TEXT NOT NULL UNIQUE,
    device_fingerprint TEXT NOT NULL,
    requesting_ip INET NOT NULL,
    requesting_user_agent TEXT,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected', 'expired')),
    approved_by_user_id UUID REFERENCES users(id),
    approved_by_device_id UUID REFERENCES user_devices(id),
    signature TEXT,
    granted_session_id UUID REFERENCES sessions(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '2 minutes'
);
CREATE INDEX idx_qr_login_nonce ON qr_login_challenges(session_nonce) WHERE status = 'pending';
CREATE INDEX idx_qr_login_expires ON qr_login_challenges(expires_at);
```

#### Push Notification Tokens

```sql
CREATE TABLE push_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id UUID REFERENCES user_devices(id) ON DELETE SET NULL,
    platform TEXT NOT NULL CHECK (platform IN ('android', 'ios', 'web')),
    token TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, token)
);
CREATE INDEX idx_push_tokens_user ON push_tokens(user_id);

ALTER TABLE user_devices ADD COLUMN device_type TEXT DEFAULT 'desktop'
    CHECK (device_type IN ('desktop', 'android', 'ios', 'web'));
```

### QR Challenge-Response Protocol

```
Desktop                    Server                     Mobile (authenticated)
   |                         |                              |
   |-- POST /qr/challenge -->|                              |
   |<-- {nonce, url, ts} ----|                              |
   | [Display QR]            |                              |
   |                         |                              | [Scan QR]
   |                         |                              | [Show: "Sign in Chrome/Win?"]
   |                         |<-- POST /qr/approve ---------|
   |                         |    {nonce, sig, device_id}    |
   |                         | [Verify Ed25519 sig of nonce  |
   |                         |  against device identity key] |
   |                         | [Create session for desktop]  |
   | [Poll: GET /qr/{nonce}] |                              |
   |<-- {approved, tokens} --|                              |
```

- **Signature:** Ed25519 sign `session_nonce` bytes using device's `identity_key_ed25519` private key
- **Nonce:** 32 random bytes, hex-encoded (64 chars)
- **TTL:** 2 minutes, one-use
- **Rate limit:** 5 per IP per minute
- **Replay protection:** Nonce consumed atomically on approve

### Notification Channels

- `messages` — default importance
- `voice_calls` — high importance, full-screen intent
- `mentions` — high importance
- `friend_requests` — default importance

### Push Token Lifecycle

- Server cron (hourly): delete `push_tokens` with `updated_at < NOW() - INTERVAL '30 days'`
- On FCM send failure (HTTP 404 / token invalid): delete stale token immediately
- Mobile `onNewToken` callback: POST `/api/push/refresh` with old + new token for atomic swap

---

## 5. Security & Platform

### Token Storage

- Access token (JWT, 15min): EncryptedSharedPreferences, loaded into memory on app resume
- Refresh token (7-day): EncryptedSharedPreferences, used to refresh expired access tokens
- Both backed by AndroidKeyStore master key

### App Hardening

- `android:allowBackup="false"`
- `android:usesCleartextTraffic="false"`
- Certificate pinning for production server
- ProGuard/R8 for release builds

### Android Permissions

| Permission | Used by | Milestone |
|---|---|---|
| `INTERNET`, `ACCESS_NETWORK_STATE` | Networking | M1 |
| `RECORD_AUDIO` | Voice chat | M1 |
| `BLUETOOTH`, `BLUETOOTH_CONNECT` | Audio routing | M1 |
| `FOREGROUND_SERVICE`, `FOREGROUND_SERVICE_MICROPHONE` | Voice call service | M1 |
| `POST_NOTIFICATIONS` (Android 13+) | Push notifications | M2 |
| `CAMERA` | QR scanning | M2 |

### Voice

- Audio focus management (duck/pause other apps during voice)
- Bluetooth SCO routing via `AudioManager`
- Wired headset detection
- Foreground notification with mute/disconnect controls

### WebSocket Resilience

- Auto-reconnect with exponential backoff (1s, 2s, 4s, 8s, max 30s)
- Re-authenticate on reconnect (token refresh if expired)
- Connectivity listener — pause reconnect when offline, resume on network change

### Performance Targets

- Cold start to guild list: <4s
- RAM (idle, text only): <120MB
- RAM (voice active): <180MB
- Battery: <5% per hour idle with WebSocket connected

---

## 6. Changes from Feb 1 Design

### Removed Entirely

- `vc-mobile-core` Rust crate with UniFFI scaffolding
- Kotlin/Swift shared core concept
- iOS app (all phases)
- Video/webcam on mobile
- FIDO2/Passkey auth (WebAuthn server endpoints, tables, Google Play Services FIDO2)
- QR Device Linking (endorsement protocol, key transfer encryption, server endpoints)
- Proximity LAN Transfer (mDNS, animated QR, relay server, TLS-PSK)
- Room database / full offline support
- 3 of 4 DB migrations (device_endorsements, webauthn_credentials/challenges, proximity_transfers)

### Simplified

- Networking: Kotlin-native instead of Rust core
- E2EE: thin JNI vodozemac wrapper instead of full UniFFI `MobileCryptoManager`
- MFA: 1 mechanism instead of 4
- Offline: token cache only instead of encrypted SQLite store
- Phasing: 2 milestones instead of 4 phases

### Kept As-Is

- Jetpack Compose + Kotlin + Material 3
- Hilt DI, Navigation Compose
- stream-webrtc-android for voice
- FCM for push notifications
- EncryptedSharedPreferences for token storage
- Same WebSocket protocol, same REST API, same voice signaling
- QR Challenge-Response (server endpoints + migration)
- Push notification server module
- VoiceCallService foreground service pattern
- Android notification channels

---

## 7. Critical Existing Files

| File | Relevance |
|------|-----------|
| `shared/vc-crypto/src/olm.rs` | Reference for JNI vodozemac wrapper (M2) |
| `shared/vc-crypto/src/recovery.rs` | Recovery key pattern reference |
| `client/src-tauri/src/crypto/store.rs` | Encrypted store pattern reference |
| `server/src/auth/mod.rs` | Auth router — add QR endpoints (M2) |
| `server/src/auth/handlers.rs` | Login/MFA handlers — reference for QR flow |
| `server/src/auth/jwt.rs` | JWT generation — reuse for QR-approved sessions |
| `server/src/ws/mod.rs` | WebSocket protocol — same for mobile |
| `server/src/voice/sfu.rs` | SFU — works unchanged with mobile clients |
| `server/src/ratelimit/mod.rs` | Rate limiter — add new categories (M2) |
| `shared/vc-common/` | WebSocket event types — implement matching Kotlin DTOs |

## 8. Verification

- **Android:** `./gradlew testDebugUnitTest` — ViewModel unit tests, repository tests
- **Android UI:** `./gradlew connectedAndroidTest` — Compose UI tests
- **Server (M2):** `cargo test -p vc-server --test qr_auth --test push` — integration tests for new endpoints
- **Cross-device E2E:** Manual test: display QR on desktop, scan with Android app, verify login completes

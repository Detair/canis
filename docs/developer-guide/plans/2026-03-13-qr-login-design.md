# QR Code Mobile Login — Design

**Goal:** Let an authenticated desktop user generate a QR code that an Android device can scan to instantly log in, without manually typing server URL or credentials.

**Motivation:** Needed to test the Android M1 app against a running server without tedious manual configuration.

**Scope:** Simplified one-time token flow. No Ed25519 device keys, no polling, no persistent device linking. The full challenge-response protocol from the mobile app design doc is deferred to Milestone 2.

---

## Flow

```
Desktop (authenticated)          Server                    Android (unauthenticated)
   |                               |                              |
   |-- POST /auth/qr/create ------>|                              |
   |<-- {token, expires_in: 120} --|                              |
   | [Display QR]                  | [Store qr_login:{token}      |
   |                               |  -> user_id in Valkey,       |
   |                               |  TTL 120s]                   |
   |                               |                              | [Scan QR]
   |                               |                              | [Parse kaiku://qr/login?
   |                               |                              |  server=...&token=...]
   |                               |<-- POST /auth/qr/redeem -----|
   |                               |    {token}                   |
   |                               | [Lookup & delete token]      |
   |                               | [Issue access+refresh tokens]|
   |                               |-- AuthResponse ------------->|
   |                               |                              | [Store tokens, navigate home]
```

## QR Payload

URI format: `kaiku://qr/login?server=<url-encoded-server-url>&token=<uuid>`

Example: `kaiku://qr/login?server=https%3A%2F%2Fchat.example.com&token=a1b2c3d4-e5f6-7890-abcd-ef1234567890`

## Server

### `POST /auth/qr/create` (authenticated)

Creates a one-time login token for the calling user.

- Generates a UUID v4 token
- Stores in Valkey: key `qr_login:{token}` with value `{user_id}`, TTL 120 seconds
- Rate limit: 5 per user per minute
- Response: `{ "token": "<uuid>", "expires_in": 120 }`

### `POST /auth/qr/redeem` (unauthenticated)

Redeems a one-time token for a full auth session.

- Body: `{ "token": "<uuid>" }`
- Looks up `qr_login:{token}` in Valkey
- If found: atomically deletes key (one-use), issues fresh access + refresh token pair for the stored user_id
- Response: standard `AuthResponse` (`access_token`, `refresh_token`, `expires_in`, `token_type`)
- If not found or expired: `401 { "error": "invalid_token", "message": "Invalid or expired QR code" }`

### Storage

Valkey only — no new database tables. Tokens auto-expire after 2 minutes via TTL.

## Desktop UI

**Location:** Settings view, Account section.

"Link Mobile Device" button opens a modal containing:
- QR code rendered client-side (using the existing `qrcode` npm package)
- Instructional text: "Scan this code with the Kaiku mobile app"
- Countdown timer showing seconds remaining
- On expiry: "Code expired" message + "Generate new code" button
- Close button

The modal calls `POST /auth/qr/create` on open and re-calls on "Generate new code".

## Android UI

**Scanner library:** ML Kit Barcode Scanning (com.google.mlkit:barcode-scanning). Works offline, no Google Play Services dependency.

**Requires:** `CAMERA` permission (runtime request on first scan).

**Entry points:**
1. **ServerUrlScreen** — "Scan QR Code" button below the URL input. Primary entry for first-time setup.
2. **SettingsScreen** — "Scan QR Code" option for re-linking or switching accounts.

**Scan flow:**
1. Open camera preview with ML Kit barcode detector
2. On detecting a `kaiku://qr/login` URI, parse `server` and `token` parameters
3. Save server URL to TokenStorage
4. Call `POST {server}/auth/qr/redeem` with `{token}`
5. On success: store tokens, call `/auth/me`, set auth state, navigate to home
6. On failure: show error ("QR code expired or already used"), allow retry

## Security

- **One-use:** Token deleted atomically on redeem (Valkey GET + DEL in one operation)
- **Short-lived:** 2-minute TTL, auto-expires in Valkey
- **Rate-limited:** 5 token creations per user per minute
- **No secret in QR:** The token alone is useless after redemption or expiry
- **HTTPS only:** Server URL in QR must use HTTPS (enforced client-side, with localhost exception for development)

## Non-Goals

- Device key management (Ed25519) — deferred to Milestone 2
- Desktop polling for approval — not needed in this simplified flow
- Push notification to mobile — mobile initiates by scanning
- Persistent device linking / endorsements — deferred
- iOS support — Android only for now

# Presigned URL Redirect Design

**Date:** 2026-03-15

**Goal:** Serve all user-uploaded files (avatars, banners, icons, attachments) via a server redirect endpoint that generates fresh presigned S3 URLs, eliminating broken images and removing direct S3 URL exposure.

## Architecture

```
Browser: <img src="/api/files/avatars/uuid/file.png">
         ↓
Server:  GET /api/files/{key...}
         → Generate presigned URL via public S3 client
         → 302 Location: https://kaiku.pmind.de/s3/voicechat/avatars/uuid/file.png?X-Amz-Signature=...
         → Cache-Control: public, max-age=3500
         ↓
Browser: Follows redirect, loads bytes directly from S3 (via Caddy proxy)
         Caches image for ~1 hour, then re-requests /api/files/... for fresh redirect
```

## Dual S3 Client

`S3Client` gets a second AWS SDK `Client` configured with the public endpoint for presigning:

```rust
pub struct S3Client {
    client: Client,           // internal endpoint — uploads, deletes, streaming
    presign_client: Client,   // public endpoint — presigned URL generation
    bucket: String,
    presign_expiry: Duration,
}
```

- `S3_ENDPOINT` (internal): `http://rustfs:9000` — used for uploads/deletes
- `S3_PUBLIC_URL` (public): `https://kaiku.pmind.de/s3` — used as endpoint for presign client
- When `S3_PUBLIC_URL` is unset (dev mode), presign client = internal client (localhost is browser-reachable)

## Redirect Endpoint

```
GET /api/files/{key...}
```

- No auth required (URLs are shared in messages, profiles)
- Generates presigned URL from the S3 key using the public presign client
- Returns `302 Found` with `Location: <presigned URL>`
- Sets `Cache-Control: public, max-age=3500` (just under 1h presign expiry)
- Rate limited to prevent abuse (Read category)

## DB Storage Change

All file URL columns switch from storing full URLs to storing **S3 keys only**:

| Column | Before | After |
|--------|--------|-------|
| `users.avatar_url` | `https://kaiku.pmind.de/s3/voicechat/avatars/uuid/file.png` | `avatars/uuid/file.png` |
| `guilds.icon_url` | `http://rustfs:9000/voicechat/icons/uuid/file.png` | `icons/uuid/file.png` |
| `guilds.banner_url` | `http://rustfs:9000/voicechat/banners/uuid/file.png` | `banners/uuid/file.png` |

## API Response Transformation

When the server serializes a user profile, guild, or message with file URLs, it transforms S3 keys into redirect endpoint URLs:

```
DB: avatars/uuid/file.png
API response: /api/files/avatars/uuid/file.png
Browser resolves to: https://kaiku.pmind.de/api/files/avatars/uuid/file.png
```

This happens in the response serialization — a helper function `file_url(s3_key) -> String` that prepends `/api/files/`.

## Upload Flow (No Change to Upload Path)

```
Client → POST /auth/me/avatar (multipart)
Server → Upload to S3 via internal client (http://rustfs:9000)
Server → Store S3 key in DB: "avatars/uuid/timestamp_file.png"
Server → Return user profile with avatar_url: "/api/files/avatars/uuid/timestamp_file.png"
```

## What Changes

| Component | Change |
|-----------|--------|
| `s3.rs` | Add `presign_client` field, build from `S3_PUBLIC_URL` |
| `s3.rs` | `presign_get()` uses `presign_client` instead of `client` |
| `config.rs` | `S3_PUBLIC_URL` already added |
| `api/mod.rs` | Add `GET /api/files/{key...}` redirect endpoint |
| `auth/handlers.rs` | Avatar upload stores key only, not full URL |
| `guild/handlers.rs` | Banner/icon upload stores key only |
| `db/queries.rs` or response types | Add `file_url()` helper for response serialization |
| Migration | Convert existing full URLs to S3 keys in avatar_url/icon_url/banner_url |
| Attachment presigned endpoint | Use `presign_client` (already works, just needs public client) |

## What Stays the Same

- Emoji serving (`/api/guilds/{id}/emojis/{id}/image`) — small files, server-proxied, cacheable
- Primary attachment download (`/api/messages/attachments/{id}`) — server-proxied streaming
- Upload flow — still goes through server to internal S3
- Client Avatar/Image components — no changes (just receives different URL format)

## Migration

SQL migration to strip full URLs down to S3 keys:

```sql
UPDATE users SET avatar_url = regexp_replace(avatar_url, '^https?://[^/]+/[^/]+/', '') WHERE avatar_url IS NOT NULL;
UPDATE guilds SET icon_url = regexp_replace(icon_url, '^https?://[^/]+/[^/]+/', '') WHERE icon_url IS NOT NULL;
UPDATE guilds SET banner_url = regexp_replace(banner_url, '^https?://[^/]+/[^/]+/', '') WHERE banner_url IS NOT NULL;
```

## Performance

- **No double bandwidth** — server sends 302 (< 1KB), browser fetches from S3 directly
- **Browser caching** — `Cache-Control: public, max-age=3500` means one redirect per image per hour
- **No broken images** — redirect always generates fresh presigned URL
- **Presign cost** — negligible (local crypto operation, no network call to S3)

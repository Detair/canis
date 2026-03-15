# Presigned URL Redirect Implementation Plan

**Goal:** Serve all user-uploaded files via a 302 redirect endpoint that generates fresh presigned S3 URLs, using a dual S3 client (internal for uploads, public for presigning).

**Architecture:** Add a `presign_client` to `S3Client` built from `S3_PUBLIC_URL`. Add `GET /api/files/{key...}` redirect endpoint. Store S3 keys (not full URLs) in DB. Transform keys to `/api/files/` URLs in API responses.

**Tech Stack:** Rust, aws-sdk-s3 (presigning), axum (302 redirect), PostgreSQL (migration)

**Design doc:** `docs/developer-guide/plans/2026-03-15-presigned-url-redirect-design.md`

---

### Task 1: Add presign_client to S3Client

**Files:**
- Modify: `server/src/chat/s3.rs`

**Step 1: Add presign_client field to struct**

Change the struct from:
```rust
pub struct S3Client {
    client: Client,
    bucket: String,
    presign_expiry: Duration,
}
```
to:
```rust
pub struct S3Client {
    client: Client,
    presign_client: Client,
    bucket: String,
    presign_expiry: Duration,
}
```

**Step 2: Build presign_client in constructor**

After `let client = Client::from_conf(s3_config);` (line 101), build a second client when `S3_PUBLIC_URL` is set:

```rust
// Build presign client with public endpoint (for browser-accessible presigned URLs)
let presign_client = if let Some(public_url) = &config.s3_public_url {
    let mut presign_builder = aws_sdk_s3::Config::builder()
        .region(Region::new(
            std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
        ))
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .identity_cache(IdentityCache::no_cache())
        .sleep_impl(Arc::new(TokioSleep::new()))
        .behavior_version_latest()
        .endpoint_url(public_url)
        .force_path_style(true);

    // Same credentials as internal client
    let access_key = config.s3_access_key.clone()
        .or_else(|| std::env::var("AWS_ACCESS_KEY_ID").ok());
    let secret_key = config.s3_secret_key.clone()
        .or_else(|| std::env::var("AWS_SECRET_ACCESS_KEY").ok());
    if let (Some(ak), Some(sk)) = (access_key, secret_key) {
        let creds = Credentials::new(ak, sk, None, None, "environment");
        presign_builder = presign_builder
            .credentials_provider(SharedCredentialsProvider::new(creds));
    }

    info!(public_url = %public_url, "Presign client initialized with public endpoint");
    Client::from_conf(presign_builder.build())
} else {
    client.clone()
};
```

Update the `Ok(Self { ... })` to include `presign_client`.

**Step 3: Update presign_get to use presign_client**

Change `self.client` to `self.presign_client` in the `presign_get` method (line ~214):

```rust
let presign_future = self
    .presign_client  // was: self.client
    .get_object()
    .bucket(&self.bucket)
    .key(key)
    .presigned(presign_config);
```

**Step 4: Add public getter for bucket name**

```rust
pub fn bucket(&self) -> &str {
    &self.bucket
}
```

**Step 5: Verify compilation**

```bash
SQLX_OFFLINE=true cargo check -p vc-server
```

**Step 6: Commit**

```
feat(infra): add dual S3 client with public presign endpoint
```

---

### Task 2: Add file redirect endpoint

**Files:**
- Create: `server/src/api/files.rs`
- Modify: `server/src/api/mod.rs`

**Step 1: Create the redirect handler**

Create `server/src/api/files.rs`:

```rust
//! File redirect endpoint — generates presigned S3 URLs on-the-fly.
//!
//! GET /api/files/{key...} → 302 redirect to presigned S3 URL

use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;

use crate::AppState;

/// Redirect to a presigned S3 URL for the given file key.
///
/// Returns 302 with Cache-Control so browsers cache and re-request
/// before the presigned URL expires.
pub async fn redirect(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let s3 = match &state.s3 {
        Some(s3) => s3,
        None => return (StatusCode::SERVICE_UNAVAILABLE, "File storage not configured").into_response(),
    };

    match s3.presign_get(&key).await {
        Ok(presigned_url) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::LOCATION, presigned_url.parse().unwrap());
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=3500".parse().unwrap(),
            );
            (StatusCode::FOUND, headers, "").into_response()
        }
        Err(e) => {
            tracing::warn!(key = %key, error = %e, "Failed to generate presigned URL");
            (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    }
}
```

**Step 2: Register the route**

In `server/src/api/mod.rs`, add `pub mod files;` and register the route in the public router (no auth required, like attachment downloads):

```rust
.route("/api/files/*key", get(files::redirect))
```

Add it near the `messages_public_router` merge point.

**Step 3: Verify compilation**

```bash
SQLX_OFFLINE=true cargo check -p vc-server
```

**Step 4: Commit**

```
feat(api): add /api/files/{key} presigned URL redirect endpoint
```

---

### Task 3: Add file_url helper and update upload handlers

**Files:**
- Modify: `server/src/auth/handlers.rs` (avatar upload)
- Modify: `server/src/guild/handlers.rs` (banner upload)

**Step 1: Create a file_url helper**

Add to `server/src/api/files.rs` (or a shared util):

```rust
/// Convert an S3 key to an API file URL.
///
/// Returns `/api/files/{key}` — the redirect endpoint that generates presigned URLs.
pub fn file_url(s3_key: &str) -> String {
    format!("/api/files/{s3_key}")
}
```

**Step 2: Update avatar upload handler**

In `server/src/auth/handlers.rs`, replace the entire URL construction block (the `if let Some(public_url)` / `else if` chain) with:

```rust
use crate::api::files::file_url;

let url = file_url(&key);
```

**Step 3: Update guild banner upload handler**

Same change in `server/src/guild/handlers.rs` — replace the URL construction block with `let url = file_url(&key);`.

**Step 4: Check for guild icon upload handler**

Search for icon upload and apply the same fix if it exists.

**Step 5: Verify compilation and tests**

```bash
SQLX_OFFLINE=true cargo check -p vc-server
SQLX_OFFLINE=true cargo test -p vc-server -- voice 2>&1 | grep "test result"
```

**Step 6: Commit**

```
feat(api): switch avatar and banner uploads to file_url redirect
```

---

### Task 4: Database migration — convert URLs to S3 keys

**Files:**
- Create: `server/migrations/YYYYMMDD000000_convert_file_urls_to_s3_keys.sql`

**Step 1: Write the migration**

```sql
-- Convert full S3 URLs to bare S3 keys in file URL columns.
-- Strips the protocol + host + bucket prefix, leaving just the key path.
-- Handles: http://rustfs:9000/voicechat/..., https://kaiku.pmind.de/s3/voicechat/...,
--          /api/files/..., /voicechat/...

-- Users avatar_url
UPDATE users
SET avatar_url = regexp_replace(avatar_url, '^https?://[^/]+/[^/]+/', '')
WHERE avatar_url IS NOT NULL
  AND avatar_url LIKE 'http%';

UPDATE users
SET avatar_url = regexp_replace(avatar_url, '^/api/files/', '')
WHERE avatar_url IS NOT NULL
  AND avatar_url LIKE '/api/files/%';

-- Guilds icon_url
UPDATE guilds
SET icon_url = regexp_replace(icon_url, '^https?://[^/]+/[^/]+/', '')
WHERE icon_url IS NOT NULL
  AND icon_url LIKE 'http%';

-- Guilds banner_url
UPDATE guilds
SET banner_url = regexp_replace(banner_url, '^https?://[^/]+/[^/]+/', '')
WHERE banner_url IS NOT NULL
  AND banner_url LIKE 'http%';
```

**Step 2: Update sqlx offline cache**

```bash
DATABASE_URL="postgresql://voicechat:voicechat_dev@localhost:5433/voicechat" cargo sqlx prepare --workspace
```

(Skip if no query changes — this migration is data-only, no schema changes.)

**Step 3: Commit**

```
feat(db): migrate file URLs to S3 keys for presigned redirect
```

---

### Task 5: Transform S3 keys to file_urls in API responses

**Files:**
- Modify: Response serialization in `server/src/auth/handlers.rs`
- Modify: Response serialization in `server/src/guild/handlers.rs`
- Modify: Any place that reads `avatar_url`/`icon_url`/`banner_url` from DB and returns it

**Step 1: Find all places that return avatar_url to clients**

Search for `avatar_url` in API response structs and map them through `file_url()`:

```rust
// Before:
avatar_url: user.avatar_url,

// After:
avatar_url: user.avatar_url.as_deref().map(file_url),
```

Apply this pattern wherever `avatar_url`, `icon_url`, or `banner_url` is returned in a JSON response.

**Step 2: Verify compilation and existing tests**

```bash
SQLX_OFFLINE=true cargo check -p vc-server
cd client && bun run test:run
```

**Step 3: Commit**

```
feat(api): transform S3 keys to redirect URLs in API responses
```

---

### Task 6: Update .env and deploy

**Files:**
- Modify: `.env.example`
- Modify: `infra/compose/docker-compose.override.yml` (on VPS)

**Step 1: Ensure S3_PUBLIC_URL is documented**

Already in `.env.example` from earlier work. Verify it's there.

**Step 2: Remove bucket-public policy**

The bucket no longer needs public read access since presigned URLs include auth:

```bash
docker run --rm --network compose_voicechat \
  -e MC_HOST_rs="http://${S3_KEY}:${S3_SECRET}@rustfs:9000" \
  minio/mc anonymous set none rs/voicechat
```

**Step 3: Rebuild and deploy server**

```bash
cd /opt/kaiku && git pull
cd infra/compose && docker compose build server
docker compose --profile monitoring up -d --force-recreate server
```

**Step 4: Run migration (automatic on startup)**

Server runs migrations on start. Verify in logs:
```bash
docker logs canis-server --tail 10 | grep migration
```

**Step 5: Test**

```bash
# Health check
curl -sf https://kaiku.pmind.de/health

# Test file redirect
curl -sv https://kaiku.pmind.de/api/files/avatars/58b286b0-f4df-45fe-91c3-5a4c55067905/1773590387_indiana_tux.png 2>&1 | grep "302\|Location"
```

**Step 6: Commit deployment notes**

```
docs(infra): update deployment for presigned URL redirect
```

---

### Task 7: CHANGELOG and final verification

**Step 1: Add CHANGELOG entry**

Under `[Unreleased] → ### Changed`:

```
- File URLs (avatars, banners) now use presigned S3 URLs via redirect — no more broken images after session timeout, no direct S3 exposure
```

**Step 2: Run full checks**

```bash
cargo fmt --check
SQLX_OFFLINE=true cargo clippy -p vc-server -- -D warnings
cd client && bun run test:run
```

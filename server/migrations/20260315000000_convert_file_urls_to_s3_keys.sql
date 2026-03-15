-- Convert full S3 URLs to bare S3 keys in file URL columns.
-- After this migration, avatar_url/icon_url/banner_url store only
-- the S3 key (e.g., "avatars/uuid/file.png"), not the full URL.
-- The /api/files/ redirect endpoint generates presigned URLs on-the-fly.

-- Users: strip http(s)://host/bucket/ prefix from avatar_url
UPDATE users
SET avatar_url = regexp_replace(avatar_url, '^https?://[^/]+/[^/]+/', '')
WHERE avatar_url IS NOT NULL
  AND avatar_url LIKE 'http%';

-- Users: strip /api/files/ prefix (already migrated to redirect format)
UPDATE users
SET avatar_url = regexp_replace(avatar_url, '^/api/files/', '')
WHERE avatar_url IS NOT NULL
  AND avatar_url LIKE '/api/files/%';

-- Guilds icon_url
UPDATE guilds
SET icon_url = regexp_replace(icon_url, '^https?://[^/]+/[^/]+/', '')
WHERE icon_url IS NOT NULL
  AND icon_url LIKE 'http%';

UPDATE guilds
SET icon_url = regexp_replace(icon_url, '^/api/files/', '')
WHERE icon_url IS NOT NULL
  AND icon_url LIKE '/api/files/%';

-- Guilds banner_url
UPDATE guilds
SET banner_url = regexp_replace(banner_url, '^https?://[^/]+/[^/]+/', '')
WHERE banner_url IS NOT NULL
  AND banner_url LIKE 'http%';

UPDATE guilds
SET banner_url = regexp_replace(banner_url, '^/api/files/', '')
WHERE banner_url IS NOT NULL
  AND banner_url LIKE '/api/files/%';

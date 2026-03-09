-- Update default max_screen_shares from 1 to 6 for multi-stream support
ALTER TABLE channels ALTER COLUMN max_screen_shares SET DEFAULT 6;

COMMENT ON COLUMN channels.max_screen_shares IS 'Maximum concurrent screen shares in this channel (default 6, supports multi-stream)';

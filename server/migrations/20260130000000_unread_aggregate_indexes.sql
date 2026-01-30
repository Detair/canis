-- Unread Aggregator Performance Indexes
-- Migration: 20260130000000_unread_aggregate_indexes
-- Purpose: Optimize the unread aggregator query for fast performance

-- ============================================================================
-- Channel Read State Composite Index
-- ============================================================================
-- While we have PRIMARY KEY (user_id, channel_id), the unread query joins on:
-- LEFT JOIN channel_read_state crs ON crs.channel_id = c.id AND crs.user_id = $1
--
-- This composite index with channel_id first allows efficient lookups during
-- the join operation, especially when filtering by a specific user_id.
-- The INCLUDE clause adds last_read_at to the index (covering index) to avoid
-- additional table lookups.

CREATE INDEX IF NOT EXISTS idx_channel_read_state_channel_user_covering
    ON channel_read_state(channel_id, user_id)
    INCLUDE (last_read_at);

-- ============================================================================
-- Analysis
-- ============================================================================
-- Query pattern from get_unread_aggregate:
--
-- Guild Query:
--   FROM guild_members gm                          -- Uses idx_guild_members_user_guild(user_id, guild_id)
--   INNER JOIN guilds g ON g.id = gm.guild_id     -- Uses primary key
--   INNER JOIN channels c ON c.guild_id = g.id    -- Uses primary key
--   LEFT JOIN channel_read_state crs              -- Uses new composite index
--       ON crs.channel_id = c.id
--       AND crs.user_id = $1
--   LEFT JOIN messages m ON m.channel_id = c.id   -- Uses idx_messages_active
--       AND m.deleted_at IS NULL
--       AND (crs.last_read_at IS NULL OR m.created_at > crs.last_read_at)
--
-- DM Query:
--   FROM dm_participants dp                       -- Uses primary key
--   INNER JOIN channels c ON c.id = dp.channel_id -- Uses primary key
--   LEFT JOIN channel_read_state crs              -- Uses new composite index
--       ON crs.channel_id = c.id
--       AND crs.user_id = $1
--   LEFT JOIN messages m ON m.channel_id = c.id   -- Uses idx_messages_active
--       AND m.deleted_at IS NULL
--       AND m.user_id != $1
--       AND (crs.last_read_at IS NULL OR m.created_at > crs.last_read_at)
--
-- All major joins are now covered by appropriate indexes.

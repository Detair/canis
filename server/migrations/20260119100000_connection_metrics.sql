-- User Connectivity Monitor: TimescaleDB Schema
-- Stores connection quality metrics for voice channels with automatic aggregation

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Raw metrics table (hypertable)
-- Stores per-second connection quality samples
CREATE TABLE connection_metrics (
    time        TIMESTAMPTZ NOT NULL,
    user_id     UUID NOT NULL,
    session_id  UUID NOT NULL,
    channel_id  UUID NOT NULL,
    guild_id    UUID,                    -- NULL for DM calls
    latency_ms  SMALLINT NOT NULL,       -- Round-trip latency in milliseconds
    packet_loss REAL NOT NULL,           -- Packet loss ratio (0.0 - 1.0)
    jitter_ms   SMALLINT NOT NULL,       -- Jitter in milliseconds
    quality     SMALLINT NOT NULL        -- Quality score (0-3: 0=poor, 1=fair, 2=good, 3=excellent)
);

-- Convert to hypertable for time-series optimization
SELECT create_hypertable('connection_metrics', 'time');

-- Indexes for common queries
CREATE INDEX idx_metrics_user_time ON connection_metrics (user_id, time DESC);
CREATE INDEX idx_metrics_session ON connection_metrics (session_id);

-- Row-Level Security: Users can only read their own metrics
ALTER TABLE connection_metrics ENABLE ROW LEVEL SECURITY;

CREATE POLICY user_own_metrics ON connection_metrics
    FOR SELECT
    USING (user_id = current_setting('app.current_user_id', true)::UUID);

-- Session summary table
-- Aggregated statistics for completed voice sessions
CREATE TABLE connection_sessions (
    id            UUID PRIMARY KEY,
    user_id       UUID NOT NULL,
    channel_id    UUID NOT NULL,
    guild_id      UUID,                  -- NULL for DM calls
    started_at    TIMESTAMPTZ NOT NULL,
    ended_at      TIMESTAMPTZ NOT NULL,
    avg_latency   SMALLINT,              -- Average latency over session
    avg_loss      REAL,                  -- Average packet loss over session
    avg_jitter    SMALLINT,              -- Average jitter over session
    worst_quality SMALLINT               -- Worst quality score observed
);

CREATE INDEX idx_sessions_user_time ON connection_sessions (user_id, started_at DESC);

-- Row-Level Security: Users can only read their own sessions
ALTER TABLE connection_sessions ENABLE ROW LEVEL SECURITY;

CREATE POLICY user_own_sessions ON connection_sessions
    FOR SELECT
    USING (user_id = current_setting('app.current_user_id', true)::UUID);

-- Continuous Aggregates
-- Pre-computed aggregations for efficient dashboard queries

-- Per-minute aggregates (for real-time monitoring)
CREATE MATERIALIZED VIEW metrics_by_minute
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', time) AS bucket,
    user_id,
    AVG(latency_ms)::SMALLINT AS avg_latency,
    MAX(latency_ms) AS max_latency,
    AVG(packet_loss)::REAL AS avg_loss,
    MAX(packet_loss) AS max_loss,
    AVG(jitter_ms)::SMALLINT AS avg_jitter
FROM connection_metrics
GROUP BY bucket, user_id
WITH NO DATA;

-- Per-hour aggregates (for session history)
CREATE MATERIALIZED VIEW metrics_by_hour
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', time) AS bucket,
    user_id,
    AVG(latency_ms)::SMALLINT AS avg_latency,
    AVG(packet_loss)::REAL AS avg_loss,
    AVG(jitter_ms)::SMALLINT AS avg_jitter,
    COUNT(*) AS sample_count
FROM connection_metrics
GROUP BY bucket, user_id
WITH NO DATA;

-- Per-day aggregates (for trends)
CREATE MATERIALIZED VIEW metrics_by_day
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 day', time) AS bucket,
    user_id,
    AVG(latency_ms)::SMALLINT AS avg_latency,
    AVG(packet_loss)::REAL AS avg_loss,
    AVG(jitter_ms)::SMALLINT AS avg_jitter,
    COUNT(*) AS sample_count
FROM connection_metrics
GROUP BY bucket, user_id
WITH NO DATA;

-- Retention and Compression Policies
-- Raw data: 7 days retention, compressed after 1 day
SELECT add_retention_policy('connection_metrics', INTERVAL '7 days');
SELECT add_compression_policy('connection_metrics', INTERVAL '1 day');

-- Continuous Aggregate Refresh Policies
-- Automatically refresh aggregates on schedule

SELECT add_continuous_aggregate_policy('metrics_by_minute',
    start_offset => INTERVAL '10 minutes',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');

SELECT add_continuous_aggregate_policy('metrics_by_hour',
    start_offset => INTERVAL '2 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

SELECT add_continuous_aggregate_policy('metrics_by_day',
    start_offset => INTERVAL '2 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

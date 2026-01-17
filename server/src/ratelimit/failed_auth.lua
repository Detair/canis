-- Atomic failed auth tracking and IP blocking
-- KEYS[1] = failed_auth key
-- KEYS[2] = blocked key
-- ARGV[1] = window_secs (TTL for failure counter)
-- ARGV[2] = max_failures (threshold to trigger block)
-- ARGV[3] = block_duration_secs (TTL for block)
-- Returns: {count, is_blocked (1/0), is_newly_blocked (1/0)}

local count = redis.call('INCR', KEYS[1])

-- Set expiry on first failure (atomic with INCR via this script)
if count == 1 then
    redis.call('EXPIRE', KEYS[1], ARGV[1])
end

local max_failures = tonumber(ARGV[2])
local is_blocked = 0
local is_newly_blocked = 0

-- Check if threshold exceeded
if count >= max_failures then
    -- Check if already blocked
    local already_blocked = redis.call('EXISTS', KEYS[2])
    if already_blocked == 0 then
        -- Block the IP
        redis.call('SETEX', KEYS[2], ARGV[3], '1')
        is_newly_blocked = 1
    end
    is_blocked = 1
end

return {count, is_blocked, is_newly_blocked}

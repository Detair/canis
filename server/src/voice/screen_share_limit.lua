-- Atomic screen share limit management.
-- KEYS[1] = screenshare:limit:{channel_id}
-- ARGV[1] = max_shares
-- ARGV[2] = operation: "start" | "stop" | "check"
-- Returns: {allowed (1/0), current_count}

local key = KEYS[1]
local max = tonumber(ARGV[1])
local op = ARGV[2]

if op == "check" then
    local count = tonumber(redis.call('GET', key) or '0')
    if count >= max then return {0, count} end
    return {1, count}
elseif op == "start" then
    local count = tonumber(redis.call('GET', key) or '0')
    if count >= max then return {0, count} end
    local new = redis.call('INCR', key)
    return {1, new}
elseif op == "stop" then
    local count = tonumber(redis.call('GET', key) or '0')
    if count > 0 then
        count = redis.call('DECR', key)
    end
    return {1, count}
end

return {0, -1}

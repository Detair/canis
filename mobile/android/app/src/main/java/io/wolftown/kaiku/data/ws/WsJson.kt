package io.wolftown.kaiku.data.ws

import io.wolftown.kaiku.data.KaikuJson
import kotlinx.serialization.json.Json

/**
 * Json instance for WebSocket event (de)serialization.
 *
 * Extends [KaikuJson] with `classDiscriminator = "type"` so the server's
 * `{"type": "event_name", ...}` format maps to our sealed class hierarchy.
 */
@OptIn(kotlinx.serialization.ExperimentalSerializationApi::class)
val WsJson = Json(from = KaikuJson) {
    classDiscriminator = "type"
}

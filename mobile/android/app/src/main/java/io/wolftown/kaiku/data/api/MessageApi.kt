package io.wolftown.kaiku.data.api

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.wolftown.kaiku.domain.model.Message
import kotlinx.serialization.Serializable
import javax.inject.Inject

interface MessageApi {
    suspend fun getMessages(channelId: String, before: String? = null, limit: Int = 50): List<Message>
    suspend fun sendMessage(channelId: String, content: String): Message
    suspend fun editMessage(messageId: String, content: String): Message
    suspend fun deleteMessage(messageId: String)
    suspend fun addReaction(channelId: String, messageId: String, emoji: String)
    suspend fun removeReaction(channelId: String, messageId: String, emoji: String)
}

@Serializable
private data class SendMessageRequest(val content: String)

@Serializable
private data class EditMessageRequest(val content: String)

@Serializable
private data class ReactionRequest(val emoji: String)

class MessageApiImpl @Inject constructor(
    private val httpClient: HttpClient
) : MessageApi {

    override suspend fun getMessages(channelId: String, before: String?, limit: Int): List<Message> {
        val response = httpClient.get("/api/messages/channel/$channelId") {
            parameter("limit", limit)
            if (before != null) {
                parameter("before", before)
            }
        }

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to load messages"
            )
        }

        return response.body()
    }

    override suspend fun sendMessage(channelId: String, content: String): Message {
        val response = httpClient.post("/api/messages/channel/$channelId") {
            setBody(SendMessageRequest(content))
        }

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to send message"
            )
        }

        return response.body()
    }

    override suspend fun editMessage(messageId: String, content: String): Message {
        val response = httpClient.patch("/api/messages/$messageId") {
            setBody(EditMessageRequest(content))
        }

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to edit message"
            )
        }

        return response.body()
    }

    override suspend fun deleteMessage(messageId: String) {
        val response = httpClient.delete("/api/messages/$messageId")

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to delete message"
            )
        }
    }

    override suspend fun addReaction(channelId: String, messageId: String, emoji: String) {
        val response = httpClient.put("/api/channels/$channelId/messages/$messageId/reactions") {
            setBody(ReactionRequest(emoji))
        }

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to add reaction"
            )
        }
    }

    override suspend fun removeReaction(channelId: String, messageId: String, emoji: String) {
        val response = httpClient.delete("/api/channels/$channelId/messages/$messageId/reactions/$emoji")

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to remove reaction"
            )
        }
    }
}

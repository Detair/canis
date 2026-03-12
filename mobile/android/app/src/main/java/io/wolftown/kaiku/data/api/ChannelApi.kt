package io.wolftown.kaiku.data.api

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.wolftown.kaiku.domain.model.Channel
import javax.inject.Inject

interface ChannelApi {
    suspend fun getChannels(guildId: String): List<Channel>
    suspend fun getChannel(channelId: String): Channel
}

class ChannelApiImpl @Inject constructor(
    private val httpClient: HttpClient
) : ChannelApi {

    override suspend fun getChannels(guildId: String): List<Channel> {
        val response = httpClient.get("/api/guilds/$guildId/channels")

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to load channels"
            )
        }

        return response.body()
    }

    override suspend fun getChannel(channelId: String): Channel {
        val response = httpClient.get("/api/channels/$channelId")

        if (!response.status.isSuccess()) {
            val errorBody = runCatching { response.body<ApiErrorResponse>() }.getOrNull()
            throw ApiException(
                response.status,
                errorBody?.message ?: "Failed to load channel"
            )
        }

        return response.body()
    }
}

package io.wolftown.kaiku.data.repository

import io.wolftown.kaiku.data.api.ChannelApi
import io.wolftown.kaiku.data.api.GuildApi
import io.wolftown.kaiku.domain.model.Channel
import io.wolftown.kaiku.domain.model.Guild
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class GuildRepository @Inject constructor(
    private val guildApi: GuildApi,
    private val channelApi: ChannelApi
) {
    private val _guilds = MutableStateFlow<List<Guild>>(emptyList())
    val guilds: StateFlow<List<Guild>> = _guilds.asStateFlow()

    private val _selectedGuildId = MutableStateFlow<String?>(null)
    val selectedGuildId: StateFlow<String?> = _selectedGuildId.asStateFlow()

    private val _channels = MutableStateFlow<List<Channel>>(emptyList())
    val channels: StateFlow<List<Channel>> = _channels.asStateFlow()

    suspend fun loadGuilds() {
        val guildList = guildApi.getGuilds()
        _guilds.value = guildList
    }

    fun selectGuild(guildId: String) {
        _selectedGuildId.value = guildId
    }

    suspend fun loadChannels(guildId: String) {
        val channelList = channelApi.getChannels(guildId)
        _channels.value = channelList
    }
}

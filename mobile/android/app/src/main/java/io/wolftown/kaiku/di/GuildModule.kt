package io.wolftown.kaiku.di

import dagger.Binds
import dagger.Module
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import io.wolftown.kaiku.data.api.ChannelApi
import io.wolftown.kaiku.data.api.ChannelApiImpl
import io.wolftown.kaiku.data.api.GuildApi
import io.wolftown.kaiku.data.api.GuildApiImpl
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
abstract class GuildModule {

    @Binds
    @Singleton
    abstract fun bindGuildApi(impl: GuildApiImpl): GuildApi

    @Binds
    @Singleton
    abstract fun bindChannelApi(impl: ChannelApiImpl): ChannelApi
}

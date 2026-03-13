package io.wolftown.kaiku.domain.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
enum class UserStatus {
    @SerialName("online") ONLINE,
    @SerialName("idle") IDLE,
    @SerialName("dnd") DND,
    @SerialName("offline") OFFLINE;
}

@Serializable
data class User(
    val id: String,
    val username: String,
    val displayName: String,
    val avatarUrl: String? = null,
    val status: UserStatus = UserStatus.OFFLINE,
    val mfaEnabled: Boolean = false,
    val createdAt: String = ""
)

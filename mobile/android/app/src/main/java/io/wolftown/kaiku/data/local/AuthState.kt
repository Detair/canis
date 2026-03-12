package io.wolftown.kaiku.data.local

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class AuthState @Inject constructor() {

    private val _isLoggedIn = MutableStateFlow(false)
    val isLoggedIn: StateFlow<Boolean> = _isLoggedIn.asStateFlow()

    private val _currentUserId = MutableStateFlow<String?>(null)
    val currentUserId: StateFlow<String?> = _currentUserId.asStateFlow()

    fun setLoggedIn(userId: String) {
        _currentUserId.value = userId
        _isLoggedIn.value = true
    }

    fun setLoggedOut() {
        _isLoggedIn.value = false
        _currentUserId.value = null
    }

    fun initialize(tokenStorage: TokenStorage) {
        val token = tokenStorage.getAccessToken()
        val userId = tokenStorage.getUserId()
        val expired = tokenStorage.isAccessTokenExpired()

        if (token != null && userId != null && !expired) {
            setLoggedIn(userId)
        } else {
            setLoggedOut()
        }
    }
}

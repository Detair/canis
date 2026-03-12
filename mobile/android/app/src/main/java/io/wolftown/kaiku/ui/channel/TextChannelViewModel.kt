package io.wolftown.kaiku.ui.channel

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import io.wolftown.kaiku.data.repository.ChatRepository
import io.wolftown.kaiku.domain.model.Message
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class TextChannelViewModel @Inject constructor(
    private val chatRepository: ChatRepository,
    savedStateHandle: SavedStateHandle
) : ViewModel() {

    private val channelId: String = savedStateHandle["channelId"]!!

    val messages: StateFlow<List<Message>> = chatRepository.getMessages(channelId)
    val typingUsers: StateFlow<Set<String>> = chatRepository.getTypingUsers(channelId)

    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()

    private val _messageInput = MutableStateFlow("")
    val messageInput: StateFlow<String> = _messageInput.asStateFlow()

    init {
        chatRepository.subscribeToChannel(channelId)
        loadInitialMessages()
    }

    fun onMessageInputChanged(text: String) {
        _messageInput.value = text
        if (text.isNotEmpty()) {
            chatRepository.sendTypingIndicator(channelId)
        }
    }

    fun onSendMessage() {
        val content = _messageInput.value.trim()
        if (content.isEmpty()) return

        _messageInput.value = ""
        viewModelScope.launch {
            try {
                chatRepository.sendMessage(channelId, content)
            } catch (e: Exception) {
                // Restore input on failure so the user can retry
                _messageInput.value = content
            }
        }
    }

    fun onEditMessage(messageId: String, content: String) {
        viewModelScope.launch {
            try {
                chatRepository.editMessage(messageId, content)
            } catch (_: Exception) {
                // Edit failed silently — message retains original content
            }
        }
    }

    fun onDeleteMessage(messageId: String) {
        viewModelScope.launch {
            try {
                chatRepository.deleteMessage(channelId, messageId)
            } catch (_: Exception) {
                // Delete failed silently
            }
        }
    }

    fun onAddReaction(messageId: String, emoji: String) {
        viewModelScope.launch {
            try {
                chatRepository.addReaction(channelId, messageId, emoji)
            } catch (_: Exception) {
                // Reaction failed silently
            }
        }
    }

    fun onRemoveReaction(messageId: String, emoji: String) {
        viewModelScope.launch {
            try {
                chatRepository.removeReaction(channelId, messageId, emoji)
            } catch (_: Exception) {
                // Reaction removal failed silently
            }
        }
    }

    fun onLoadMore() {
        val oldestMessage = messages.value.firstOrNull() ?: return
        viewModelScope.launch {
            try {
                chatRepository.loadMessages(channelId, before = oldestMessage.id)
            } catch (_: Exception) {
                // Pagination failed silently
            }
        }
    }

    public override fun onCleared() {
        chatRepository.unsubscribeFromChannel(channelId)
        super.onCleared()
    }

    private fun loadInitialMessages() {
        _isLoading.value = true
        viewModelScope.launch {
            try {
                chatRepository.loadMessages(channelId)
            } catch (_: Exception) {
                // Initial load failed — messages flow stays empty
            } finally {
                _isLoading.value = false
            }
        }
    }
}

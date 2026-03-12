package io.wolftown.kaiku.ui.channel

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Send
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Message composition area with typing indicators and send button.
 *
 * @param input Current message text
 * @param channelName Channel name shown in placeholder
 * @param typingUsers Set of user display names currently typing
 * @param onInputChanged Callback when text changes
 * @param onSend Callback when the send button is pressed
 */
@Composable
fun MessageInput(
    input: String,
    channelName: String,
    typingUsers: Set<String>,
    onInputChanged: (String) -> Unit,
    onSend: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier) {
        // Typing indicator
        AnimatedVisibility(
            visible = typingUsers.isNotEmpty(),
            enter = fadeIn(),
            exit = fadeOut()
        ) {
            Text(
                text = formatTypingIndicator(typingUsers),
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp)
            )
        }

        // Input row
        Surface(
            tonalElevation = 2.dp,
            modifier = Modifier.fillMaxWidth()
        ) {
            Row(
                modifier = Modifier
                    .padding(horizontal = 8.dp, vertical = 8.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                TextField(
                    value = input,
                    onValueChange = onInputChanged,
                    placeholder = { Text("Message #$channelName") },
                    modifier = Modifier.weight(1f),
                    singleLine = false,
                    maxLines = 5,
                    colors = TextFieldDefaults.colors(
                        focusedContainerColor = MaterialTheme.colorScheme.surfaceContainerHigh,
                        unfocusedContainerColor = MaterialTheme.colorScheme.surfaceContainerHigh
                    )
                )

                Spacer(modifier = Modifier.width(8.dp))

                IconButton(
                    onClick = onSend,
                    enabled = input.isNotBlank()
                ) {
                    Icon(
                        imageVector = Icons.AutoMirrored.Filled.Send,
                        contentDescription = "Send message",
                        tint = if (input.isNotBlank()) {
                            MaterialTheme.colorScheme.primary
                        } else {
                            MaterialTheme.colorScheme.onSurfaceVariant
                        }
                    )
                }
            }
        }
    }
}

/**
 * Formats typing indicator text from a set of usernames.
 *
 * - 1 user: "Alice is typing..."
 * - 2 users: "Alice and Bob are typing..."
 * - 3+ users: "Several people are typing..."
 */
private fun formatTypingIndicator(typingUsers: Set<String>): String {
    return when (typingUsers.size) {
        0 -> ""
        1 -> "${typingUsers.first()} is typing..."
        2 -> {
            val users = typingUsers.toList()
            "${users[0]} and ${users[1]} are typing..."
        }
        else -> "Several people are typing..."
    }
}

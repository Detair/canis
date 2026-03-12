package io.wolftown.kaiku.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import io.wolftown.kaiku.data.local.AuthState
import io.wolftown.kaiku.ui.auth.LoginScreen
import io.wolftown.kaiku.ui.auth.RegisterScreen
import io.wolftown.kaiku.ui.auth.ServerUrlScreen
import io.wolftown.kaiku.ui.channel.TextChannelScreen
import io.wolftown.kaiku.ui.home.HomeScreen

@Composable
fun KaikuNavGraph(
    navController: NavHostController,
    startDestination: String,
    authState: AuthState
) {
    val currentUserId by authState.currentUserId.collectAsState()

    NavHost(navController = navController, startDestination = startDestination) {
        composable("server_url") {
            ServerUrlScreen(
                onConnectSuccess = {
                    navController.navigate("login") {
                        popUpTo("server_url") { inclusive = true }
                    }
                }
            )
        }

        composable("login") {
            LoginScreen(
                onNavigateToRegister = {
                    navController.navigate("register")
                },
                onLoginSuccess = {
                    navController.navigate("home") {
                        popUpTo("login") { inclusive = true }
                    }
                }
            )
        }

        composable("register") {
            RegisterScreen(
                onNavigateToLogin = {
                    navController.popBackStack()
                },
                onRegisterSuccess = {
                    navController.navigate("home") {
                        popUpTo("register") { inclusive = true }
                    }
                }
            )
        }

        composable("home") {
            HomeScreen(
                onNavigateToTextChannel = { channelId ->
                    navController.navigate("channel/$channelId")
                },
                onNavigateToVoiceChannel = { channelId ->
                    navController.navigate("voice/$channelId")
                }
            )
        }

        composable("channel/{channelId}") { backStackEntry ->
            val channelId = backStackEntry.arguments?.getString("channelId") ?: ""
            TextChannelScreen(
                channelName = channelId, // Will be replaced by actual name when channel data is available
                currentUserId = currentUserId ?: "",
                onNavigateBack = { navController.popBackStack() }
            )
        }

        composable("voice/{channelId}") {
            // Placeholder for VoiceChannelScreen (Task 11)
            val channelId = it.arguments?.getString("channelId") ?: ""
            PlaceholderScreen(title = "Voice Channel", subtitle = channelId)
        }
    }
}

@Composable
private fun PlaceholderScreen(title: String, subtitle: String) {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Text(text = "$title\n$subtitle")
    }
}

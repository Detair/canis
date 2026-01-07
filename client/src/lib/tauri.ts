/**
 * Tauri Command Wrappers
 * Type-safe wrappers for Tauri commands
 */

import { invoke } from "@tauri-apps/api/core";

// Types
export interface User {
  id: string;
  username: string;
  display_name: string;
  avatar_url: string | null;
}

export interface Channel {
  id: string;
  name: string;
  channel_type: "text" | "voice" | "dm";
}

export interface Message {
  id: string;
  channel_id: string;
  content: string;
  author_id: string;
}

export interface Settings {
  input_device: string | null;
  output_device: string | null;
  input_volume: number;
  output_volume: number;
  noise_suppression: boolean;
  push_to_talk: boolean;
  push_to_talk_key: string | null;
  theme: string;
}

// Auth Commands
export async function login(serverUrl: string, username: string, password: string): Promise<User> {
  return invoke("login", { request: { server_url: serverUrl, username, password } });
}

export async function logout(): Promise<void> {
  return invoke("logout");
}

export async function getCurrentUser(): Promise<User | null> {
  return invoke("get_current_user");
}

// Chat Commands
export async function getChannels(): Promise<Channel[]> {
  return invoke("get_channels");
}

export async function getMessages(channelId: string, limit?: number): Promise<Message[]> {
  return invoke("get_messages", { channelId, limit });
}

export async function sendMessage(channelId: string, content: string): Promise<Message> {
  return invoke("send_message", { channelId, content });
}

// Voice Commands
export async function joinVoice(channelId: string): Promise<void> {
  return invoke("join_voice", { channelId });
}

export async function leaveVoice(): Promise<void> {
  return invoke("leave_voice");
}

export async function setMute(muted: boolean): Promise<void> {
  return invoke("set_mute", { muted });
}

export async function setDeafen(deafened: boolean): Promise<void> {
  return invoke("set_deafen", { deafened });
}

// Settings Commands
export async function getSettings(): Promise<Settings> {
  return invoke("get_settings");
}

export async function updateSettings(settings: Settings): Promise<void> {
  return invoke("update_settings", { settings });
}

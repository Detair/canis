# VoiceChat (Canis) Roadmap

This roadmap outlines the development path from the current prototype to a production-ready, multi-tenant SaaS platform.

**Current Phase:** Phase 0 (Technical Debt & Reliability)

---

## Phase 0: Technical Debt & Reliability (Immediate)
*Goal: Fix critical performance issues and ensure basic stability before adding features.*

- [ ] **[Backend] Fix N+1 Query in Message List** `Priority: Critical`
  - Refactor `server/src/chat/messages.rs` to use bulk user fetching (`find_users_by_ids`).
  - Eliminate the loop that executes a DB query for every message.
- [ ] **[Backend] Refactor `AuthorProfile` Construction** `Priority: High`
  - Implement `From<User> for AuthorProfile` to centralize user data formatting.
  - Remove duplicate logic in `list`, `create`, and `update` handlers.
- [ ] **[Client] WebRTC Connectivity Fix** `Priority: High`
  - Verify/Implement `VoiceIceCandidate` handler in `client/src/lib/webrtc`.
  - Ensure candidates are immediately added to `RTCPeerConnection` for successful NAT traversal.

---

## Phase 1: Core Loop Stability
*Goal: Ensure the fundamental chat and voice experience is flawless and bug-free.*

- [ ] **[Tests] Message API Integration Tests**
  - Create tests for message CRUD operations to prevent regressions.
  - Verify JSON response structures.
- [ ] **[Client] Real-time Text Sync**
  - Ensure new messages via WebSocket appear instantly in the UI without refresh.
  - Handle `message.edit` and `message.delete` events live.
- [ ] **[Voice] Room State Synchronization**
  - Sync initial `RoomState` (participants, mute status) upon joining.
  - Correctly render "Who is talking" indicators in `VoiceParticipants.tsx`.
- [ ] **[Client] Audio Device Selection**
  - Add UI in `VoiceControls` to select specific Input/Output devices (Mic/Speakers).

---

## Phase 2: Rich Interactions
*Goal: Reach feature parity with basic chat apps to make it usable for daily work/play.*

- [ ] **[Media] File Attachments**
  - [ ] **Backend:** Expose and secure S3 `upload` endpoint.
  - [ ] **Client:** Implement drag-and-drop file upload in `MessageInput`.
  - [ ] **UI:** Render images/files nicely in the message list.
- [ ] **[Text] Markdown & Emojis**
  - Integrate a Markdown renderer (e.g., `solid-markdown`).
  - Add an Emoji Picker component.
- [ ] **[UX] Read Receipts / Unread Counts**
  - Track `last_read_at` timestamp per channel member.
  - Display unread notification badges on the sidebar.

---

## Phase 3: Guild Architecture & Security (The Big Refactor)
*Goal: Transform from "Simple Chat" to "Multi-Server Platform" (Discord-like architecture).*

- [ ] **[DB] Guild (Server) Entity**
  - Create `guilds` table (`id`, `name`, `owner_id`, `icon`).
  - **Migration:** Move `channels` and `roles` to belong to `guild_id`.
  - **Migration:** Refactor `channel_members` into `guild_members`.
- [ ] **[Auth] Context-Aware RBAC**
  - Implement permissions scoped to a Guild (e.g., "Admin" is only valid in Server A).
  - Define default roles (`@everyone`).
- [ ] **[UI] Server Rail**
  - Implement the vertical "Server List" sidebar on the left.
  - Build "Context Switching" logic (clicking a server loads its channels).
- [ ] **[Security] Rate Limiting**
  - Integrate `tower-governor` to protect API endpoints from spam/DoS.

---

## Phase 4: Advanced Features
*Goal: Add competitive differentiators and mobile support.*

- [ ] **[Auth] SSO / OIDC Integration**
  - Enable "Login with Google/Microsoft" via `openidconnect`.
- [ ] **[Voice] Screen Sharing**
  - Update SFU to handle multiple video tracks (Webcam + Screen).
  - Update Client UI to render "Filmstrip" or "Grid" layouts.
- [ ] **[Client] Mobile Support**
  - Adapt Tauri frontend for mobile or begin Flutter/Native implementation.

---

## Phase 5: Ecosystem & SaaS Readiness
*Goal: Open the platform to developers and prepare for massive scale.*

- [ ] **[API] Bot Ecosystem**
  - Add `is_bot` user flag.
  - Create a "Gateway" WebSocket for bot events (`MESSAGE_CREATE`).
  - Implement Slash Commands structure.
- [ ] **[Content] Custom Emojis**
  - Allow Guilds to upload custom emoji packs.
  - Update client parser to handle `<:name:id>` syntax.
- [ ] **[Voice] Multi-Stream Support**
  - Allow simultaneous Webcam and Screen Sharing from the same user.
  - Implement Simulcast (quality tiers) for bandwidth management.
- [ ] **[SaaS] Limits & Monetization Logic**
  - Enforce limits (storage, members) per Guild.
  - Prepare "Boost" logic for lifting limits.

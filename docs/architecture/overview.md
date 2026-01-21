# VoiceChat Platform - Technische Architektur

## Architektur-Übersicht

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CLIENT LAYER                                    │
│                                                                              │
│   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│   │    Windows      │  │     Linux       │  │     macOS       │             │
│   │   (Tauri 2.0)   │  │   (Tauri 2.0)   │  │   (Tauri 2.0)   │             │
│   │                 │  │                 │  │                 │             │
│   │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │             │
│   │  │  WebView  │  │  │  │  WebView  │  │  │  │  WebView  │  │             │
│   │  │ (Solid.js)│  │  │  │ (Solid.js)│  │  │  │ (Solid.js)│  │             │
│   │  └─────┬─────┘  │  │  └─────┬─────┘  │  │  └─────┬─────┘  │             │
│   │        │        │  │        │        │  │        │        │             │
│   │  ┌─────▼─────┐  │  │  ┌─────▼─────┐  │  │  ┌─────▼─────┐  │             │
│   │  │Rust Core  │  │  │  │Rust Core  │  │  │  │Rust Core  │  │             │
│   │  │• WebRTC   │  │  │  │• WebRTC   │  │  │  │• WebRTC   │  │             │
│   │  │• Audio    │  │  │  │• Audio    │  │  │  │• Audio    │  │             │
│   │  │• Crypto   │  │  │  │• Crypto   │  │  │  │• Crypto   │  │             │
│   │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │             │
│   └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│            │                    │                    │                       │
└────────────┼────────────────────┼────────────────────┼───────────────────────┘
             │                    │                    │
             └────────────────────┼────────────────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │       INTERNET            │
                    │   (TLS 1.3 encrypted)     │
                    └─────────────┬─────────────┘
                                  │
┌─────────────────────────────────┼───────────────────────────────────────────┐
│                           SERVER LAYER                                       │
│                                 │                                            │
│                    ┌────────────▼────────────┐                              │
│                    │      API Gateway        │                              │
│                    │   (Reverse Proxy)       │                              │
│                    │   • TLS Termination     │                              │
│                    │   • Rate Limiting       │                              │
│                    │   • Load Balancing      │                              │
│                    └────────────┬────────────┘                              │
│                                 │                                            │
│           ┌─────────────────────┼─────────────────────┐                     │
│           │                     │                     │                     │
│  ┌────────▼────────┐  ┌────────▼────────┐  ┌────────▼────────┐             │
│  │  Auth Service   │  │  Chat Service   │  │  Voice Service  │             │
│  │                 │  │                 │  │     (SFU)       │             │
│  │ • Local Auth    │  │ • Channels      │  │                 │             │
│  │ • OIDC/SSO      │  │ • Messages      │  │ • mediasoup/    │             │
│  │ • MFA (TOTP)    │  │ • File Upload   │  │   webrtc-rs     │             │
│  │ • Sessions      │  │ • E2EE (Olm)    │  │ • Opus Codec    │             │
│  │ • JWT Tokens    │  │ • WebSocket     │  │ • DTLS-SRTP     │             │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│           │                    │                    │                       │
│           └─────────────────────┼─────────────────────┘                     │
│                                 │                                            │
│                    ┌────────────▼────────────┐                              │
│                    │     Data Layer          │                              │
│                    │                         │                              │
│                    │  ┌─────────────────┐    │                              │
│                    │  │   PostgreSQL    │    │                              │
│                    │  │   • Users       │    │                              │
│                    │  │   • Channels    │    │                              │
│                    │  │   • Messages    │    │                              │
│                    │  │   • Permissions │    │                              │
│                    │  └─────────────────┘    │                              │
│                    │                         │                              │
│                    │  ┌─────────────────┐    │                              │
│                    │  │     Redis       │    │                              │
│                    │  │   • Sessions    │    │                              │
│                    │  │   • Presence    │    │                              │
│                    │  │   • Pub/Sub     │    │                              │
│                    │  └─────────────────┘    │                              │
│                    │                         │                              │
│                    │  ┌─────────────────┐    │                              │
│                    │  │   S3 Storage    │    │                              │
│                    │  │   • Files       │    │                              │
│                    │  │   • Avatars     │    │                              │
│                    │  │   • Backups     │    │                              │
│                    │  └─────────────────┘    │                              │
│                    └─────────────────────────┘                              │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Komponenten-Details

### 1. Client-Architektur (Tauri 2.0)

```
┌─────────────────────────────────────────────────────────────────┐
│                      TAURI CLIENT                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    FRONTEND (WebView)                       │ │
│  │                                                             │ │
│  │  Framework: Solid.js                                        │ │
│  │  Styling:   UnoCSS (Tailwind-kompatibel)                   │ │
│  │  State:     Solid Stores + Signals                          │ │
│  │  Icons:     Lucide                                          │ │
│  │                                                             │ │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │ │
│  │  │   Views     │ │ Components  │ │   Stores    │           │ │
│  │  │             │ │             │ │             │           │ │
│  │  │ • Login     │ │ • Channel   │ │ • Auth      │           │ │
│  │  │ • Channels  │ │ • Message   │ │ • Channels  │           │ │
│  │  │ • Settings  │ │ • UserList  │ │ • Messages  │           │ │
│  │  │ • Voice     │ │ • VoiceBar  │ │ • Voice     │           │ │
│  │  │             │ │ • Settings  │ │ • Settings  │           │ │
│  │  └─────────────┘ └─────────────┘ └─────────────┘           │ │
│  │                                                             │ │
│  └──────────────────────────┬─────────────────────────────────┘ │
│                             │                                    │
│                      Tauri Commands                              │
│                             │                                    │
│  ┌──────────────────────────▼─────────────────────────────────┐ │
│  │                    BACKEND (Rust)                           │ │
│  │                                                             │ │
│  │  ┌─────────────────────────────────────────────────────┐   │ │
│  │  │                   Core Modules                       │   │ │
│  │  │                                                      │   │ │
│  │  │  ┌──────────────┐  ┌──────────────┐                 │   │ │
│  │  │  │    Audio     │  │   WebRTC     │                 │   │ │
│  │  │  │              │  │              │                 │   │ │
│  │  │  │ • cpal       │  │ • webrtc-rs  │                 │   │ │
│  │  │  │ • opus       │  │ • Signaling  │                 │   │ │
│  │  │  │ • nnnoiseless│  │ • DTLS-SRTP  │                 │   │ │
│  │  │  └──────────────┘  └──────────────┘                 │   │ │
│  │  │                                                      │   │ │
│  │  │  ┌──────────────┐  ┌──────────────┐                 │   │ │
│  │  │  │    Crypto    │  │   Network    │                 │   │ │
│  │  │  │              │  │              │                 │   │ │
│  │  │  │ • vodozemac  │  │ • HTTP/REST  │                 │   │ │
│  │  │  │ • Key Store  │  │ • WebSocket  │                 │   │ │
│  │  │  │ • Keyring    │  │ • rustls     │                 │   │ │
│  │  │  └──────────────┘  └──────────────┘                 │   │ │
│  │  │                                                      │   │ │
│  │  └─────────────────────────────────────────────────────┘   │ │
│  │                                                             │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### Client-Ressourcenziele

| Metrik | Ziel | Discord zum Vergleich |
|--------|------|----------------------|
| RAM (Idle) | <80 MB | ~300-400 MB |
| RAM (Voice aktiv) | <120 MB | ~400-500 MB |
| CPU (Idle) | <1% | ~2-5% |
| CPU (Voice aktiv) | <5% | ~5-10% |
| Binärgröße | <50 MB | ~150 MB |
| Startup | <3s | ~5-10s |

---

### 2. Server-Architektur

#### Auth Service

```
┌─────────────────────────────────────────────────────────────────┐
│                       AUTH SERVICE                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Endpoints:                                                      │
│  ──────────                                                      │
│  POST   /auth/register          Lokale User-Registrierung       │
│  POST   /auth/login             Login (lokal oder SSO Start)    │
│  POST   /auth/logout            Session beenden                  │
│  POST   /auth/refresh           Access Token erneuern            │
│  GET    /auth/oidc/callback     SSO Callback Handler             │
│  POST   /auth/mfa/setup         TOTP Setup                       │
│  POST   /auth/mfa/verify        TOTP Verifizierung               │
│                                                                  │
│  Interne Funktionen:                                             │
│  ───────────────────                                             │
│  • Password Hashing (Argon2id)                                   │
│  • JWT Generation/Validation                                     │
│  • Session Management (Redis)                                    │
│  • OIDC Provider Integration                                     │
│  • JIT User Provisioning                                         │
│                                                                  │
│  Token-Strategie:                                                │
│  ────────────────                                                │
│  • Access Token:  JWT, 15 min Gültigkeit                        │
│  • Refresh Token: Opaque, 7 Tage, in Redis                      │
│  • Session:       Redis mit User-Metadata                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### Chat Service

```
┌─────────────────────────────────────────────────────────────────┐
│                       CHAT SERVICE                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  REST Endpoints:                                                 │
│  ───────────────                                                 │
│  GET    /channels                    Liste aller Channels        │
│  POST   /channels                    Channel erstellen           │
│  GET    /channels/:id                Channel Details             │
│  PATCH  /channels/:id                Channel bearbeiten          │
│  DELETE /channels/:id                Channel löschen             │
│  GET    /channels/:id/messages       Nachrichten laden           │
│  POST   /channels/:id/messages       Nachricht senden            │
│  PATCH  /messages/:id                Nachricht bearbeiten        │
│  DELETE /messages/:id                Nachricht löschen           │
│  POST   /upload                      Datei hochladen             │
│                                                                  │
│  WebSocket Events (Signaling):                                   │
│  ──────────────────────────────                                  │
│  → message.new          Neue Nachricht                           │
│  → message.edit         Nachricht bearbeitet                     │
│  → message.delete       Nachricht gelöscht                       │
│  → typing.start         User tippt                               │
│  → typing.stop          User tippt nicht mehr                    │
│  → presence.update      Online-Status geändert                   │
│  → channel.update       Channel-Änderung                         │
│                                                                  │
│  E2EE Integration:                                               │
│  ─────────────────                                               │
│  • Olm Sessions für 1:1 DMs                                      │
│  • Megolm Sessions für Gruppen-Channels                          │
│  • Key Exchange über separaten Kanal                             │
│  • Server speichert nur verschlüsselte Nachrichten               │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### Voice Service (SFU)

```
┌─────────────────────────────────────────────────────────────────┐
│                      VOICE SERVICE (SFU)                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Architektur: Selective Forwarding Unit                          │
│  ─────────────────────────────────────                           │
│                                                                  │
│     Client A          SFU Server           Client B              │
│        │                  │                   │                  │
│        │──── Offer ──────►│                   │                  │
│        │◄─── Answer ──────│                   │                  │
│        │                  │                   │                  │
│        │==== Media =======│======= Media ====►│                  │
│        │◄=== Media =======│◄====== Media =====│                  │
│        │                  │                   │                  │
│                                                                  │
│  Der SFU:                                                        │
│  • Empfängt Media von jedem Client einmal                        │
│  • Leitet an alle anderen Clients weiter                         │
│  • Kein Mixing/Transcoding (CPU-effizient)                       │
│  • Skaliert besser als Mesh für >4 User                          │
│                                                                  │
│  WebRTC Signaling (JSON-RPC über WebSocket):                     │
│  ───────────────────────────────────────────                     │
│  → voice.join           Voice-Channel beitreten                  │
│  → voice.leave          Voice-Channel verlassen                  │
│  → voice.offer          SDP Offer                                │
│  → voice.answer         SDP Answer                               │
│  → voice.ice            ICE Candidate                            │
│  → voice.mute           Selbst muten                             │
│  → voice.unmute         Selbst unmuten                           │
│  ← voice.user_joined    User ist beigetreten                     │
│  ← voice.user_left      User hat verlassen                       │
│  ← voice.speaking       User spricht                             │
│                                                                  │
│  Audio Pipeline:                                                 │
│  ──────────────                                                  │
│  Capture → Opus Encode → SRTP Encrypt → Network                  │
│  Network → SRTP Decrypt → Opus Decode → Playback                 │
│                                                                  │
│  Konfigurierbare Parameter:                                      │
│  ──────────────────────────                                      │
│  • Opus Bitrate: 24-96 kbps (default: 64 kbps)                  │
│  • Opus Frame Size: 10-60 ms (default: 20 ms)                   │
│  • Max Users pro Channel: 50-100 (default: 50)                  │
│  • Jitter Buffer: 20-200 ms (adaptive)                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 3. Datenbank-Schema (Übersicht)

```
┌─────────────────────────────────────────────────────────────────┐
│                     DATABASE SCHEMA                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐       ┌──────────────┐                        │
│  │    users     │       │   sessions   │                        │
│  ├──────────────┤       ├──────────────┤                        │
│  │ id (UUID)    │◄──────│ user_id      │                        │
│  │ username     │       │ token_hash   │                        │
│  │ display_name │       │ expires_at   │                        │
│  │ email        │       │ ip_address   │                        │
│  │ password_hash│       │ user_agent   │                        │
│  │ auth_method  │       └──────────────┘                        │
│  │ external_id  │                                                │
│  │ avatar_url   │       ┌──────────────┐                        │
│  │ status       │       │  user_keys   │                        │
│  │ mfa_secret   │       ├──────────────┤                        │
│  │ created_at   │◄──────│ user_id      │                        │
│  │ updated_at   │       │ identity_key │                        │
│  └──────┬───────┘       │ signed_prekey│                        │
│         │               │ one_time_keys│                        │
│         │               └──────────────┘                        │
│         │                                                        │
│         │               ┌──────────────┐                        │
│         │               │   channels   │                        │
│         │               ├──────────────┤                        │
│         │               │ id (UUID)    │                        │
│         │               │ name         │                        │
│         │               │ type         │◄─── voice│text│dm      │
│         │               │ category_id  │                        │
│         │               │ position     │                        │
│         │               │ topic        │                        │
│         │               │ user_limit   │                        │
│         │               │ created_at   │                        │
│         │               └──────┬───────┘                        │
│         │                      │                                 │
│         │     ┌────────────────┼────────────────┐               │
│         │     │                │                │               │
│         ▼     ▼                ▼                ▼               │
│  ┌──────────────┐       ┌──────────────┐ ┌──────────────┐       │
│  │ channel_     │       │   messages   │ │   megolm_    │       │
│  │ members      │       ├──────────────┤ │   sessions   │       │
│  ├──────────────┤       │ id (UUID)    │ ├──────────────┤       │
│  │ channel_id   │       │ channel_id   │ │ channel_id   │       │
│  │ user_id      │       │ user_id      │ │ session_id   │       │
│  │ role_id      │       │ content_enc  │◄─ verschlüsselt│       │
│  │ joined_at    │       │ nonce        │ │ sender_key   │       │
│  └──────────────┘       │ reply_to     │ │ created_at   │       │
│                         │ edited_at    │ └──────────────┘       │
│  ┌──────────────┐       │ created_at   │                        │
│  │    roles     │       └──────────────┘                        │
│  ├──────────────┤                                                │
│  │ id (UUID)    │       ┌──────────────┐                        │
│  │ name         │       │    files     │                        │
│  │ color        │       ├──────────────┤                        │
│  │ permissions  │◄─ JSONB│ id (UUID)    │                        │
│  │ position     │       │ message_id   │                        │
│  │ created_at   │       │ filename     │                        │
│  └──────────────┘       │ mime_type    │                        │
│                         │ size_bytes   │                        │
│                         │ s3_key       │                        │
│                         │ created_at   │                        │
│                         └──────────────┘                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 4. Verschlüsselungsarchitektur

```
┌─────────────────────────────────────────────────────────────────┐
│                  ENCRYPTION ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  LAYER 1: Transport (alle Verbindungen)                         │
│  ═══════════════════════════════════════                        │
│                                                                  │
│  Client ◄────── TLS 1.3 ──────► Server                          │
│                                                                  │
│  • Alle HTTP/WebSocket Verbindungen                              │
│  • Certificate Pinning im Client (optional)                      │
│  • rustls für Implementation                                     │
│                                                                  │
│  ─────────────────────────────────────────────────────────────  │
│                                                                  │
│  LAYER 2: Voice (WebRTC)                                        │
│  ═══════════════════════════════════════                        │
│                                                                  │
│  MVP: DTLS-SRTP                                                  │
│  ┌─────────┐         ┌─────────┐         ┌─────────┐           │
│  │Client A │◄─DTLS──►│   SFU   │◄─DTLS──►│Client B │           │
│  └─────────┘  SRTP   └─────────┘  SRTP   └─────────┘           │
│                          │                                       │
│                    Server sieht                                  │
│                    Media (trusted)                               │
│                                                                  │
│  Später (Paranoid Mode): MLS                                    │
│  ┌─────────┐         ┌─────────┐         ┌─────────┐           │
│  │Client A │◄─MLS────│   SFU   │────MLS─►│Client B │           │
│  └─────────┘ E2EE    └─────────┘  E2EE   └─────────┘           │
│                          │                                       │
│                    Server sieht                                  │
│                    nur Ciphertext                                │
│                                                                  │
│  ─────────────────────────────────────────────────────────────  │
│                                                                  │
│  LAYER 3: Text Messages                                         │
│  ═══════════════════════════════════════                        │
│                                                                  │
│  1:1 Direct Messages: Olm (Double Ratchet)                      │
│  ┌─────────┐                              ┌─────────┐           │
│  │ User A  │                              │ User B  │           │
│  │         │                              │         │           │
│  │ Olm     │◄────── Encrypted ──────────►│ Olm     │           │
│  │ Session │        Messages              │ Session │           │
│  └─────────┘                              └─────────┘           │
│       │                                        │                 │
│       └───► X3DH Key Agreement ◄───────────────┘                │
│             (One-time Prekeys)                                   │
│                                                                  │
│  Group Channels: Megolm                                         │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐                       │
│  │ User A  │   │ User B  │   │ User C  │                       │
│  │         │   │         │   │         │                       │
│  │ Megolm  │   │ Megolm  │   │ Megolm  │                       │
│  │ Outbound│   │ Inbound │   │ Inbound │                       │
│  │ Session │   │ Session │   │ Session │                       │
│  └────┬────┘   └────┬────┘   └────┬────┘                       │
│       │             │             │                              │
│       │     ┌───────▼───────┐     │                              │
│       └────►│ Shared Session│◄────┘                              │
│             │ (Ratchets     │                                    │
│             │  forward only)│                                    │
│             └───────────────┘                                    │
│                                                                  │
│  Key Distribution:                                              │
│  • Olm Sessions zum sicheren Key-Austausch                      │
│  • Megolm Session Keys via Olm verteilt                          │
│  • Bei User Join/Leave: Key Rotation                             │
│                                                                  │
│  ─────────────────────────────────────────────────────────────  │
│                                                                  │
│  LAYER 4: Data at Rest                                          │
│  ═══════════════════════════════════════                        │
│                                                                  │
│  • Messages: Bereits E2EE verschlüsselt gespeichert             │
│  • Files: AES-256-GCM vor S3 Upload                             │
│  • Backups: Verschlüsselt mit Server-Key                        │
│  • User Keys: Im OS Keychain (Client-side)                      │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 5. SSO/Identity Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    IDENTITY ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                     ┌─────────────────────┐                     │
│                     │    User Request     │                     │
│                     │   "Login with SSO"  │                     │
│                     └──────────┬──────────┘                     │
│                                │                                 │
│                                ▼                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                     Auth Service                             ││
│  │                                                              ││
│  │  ┌─────────────────┐         ┌─────────────────────────┐   ││
│  │  │  Local Auth     │         │    OIDC Handler         │   ││
│  │  │                 │         │                         │   ││
│  │  │ • Username/Pass │         │ • Discovery             │   ││
│  │  │ • Argon2id      │         │ • Authorization URL     │   ││
│  │  │ • TOTP MFA      │         │ • Token Exchange        │   ││
│  │  └────────┬────────┘         │ • UserInfo Endpoint     │   ││
│  │           │                  └────────────┬────────────┘   ││
│  │           │                               │                 ││
│  │           │         ┌─────────────────────┘                 ││
│  │           │         │                                        ││
│  │           │         ▼                                        ││
│  │           │    ┌────────────────────────────────────────┐   ││
│  │           │    │           SSO Providers                │   ││
│  │           │    │                                        │   ││
│  │           │    │  ┌──────────┐ ┌──────────┐ ┌────────┐ │   ││
│  │           │    │  │Authentik │ │ Keycloak │ │Azure AD│ │   ││
│  │           │    │  └──────────┘ └──────────┘ └────────┘ │   ││
│  │           │    │  ┌──────────┐ ┌──────────┐ ┌────────┐ │   ││
│  │           │    │  │  Okta    │ │  Google  │ │  LDAP  │ │   ││
│  │           │    │  └──────────┘ └──────────┘ └────────┘ │   ││
│  │           │    └───────────────────┬────────────────────┘   ││
│  │           │                        │                        ││
│  │           ▼                        ▼                        ││
│  │  ┌──────────────────────────────────────────────────────┐  ││
│  │  │              Unified User Store                       │  ││
│  │  │                                                       │  ││
│  │  │  user_id:        UUID (internal)                      │  ││
│  │  │  auth_method:    local | oidc                         │  ││
│  │  │  external_id:    SSO Subject (if OIDC)               │  ││
│  │  │  provider:       authentik | keycloak | ... (if OIDC)│  ││
│  │  │  username:       Unique, for mentions                 │  ││
│  │  │  display_name:   From SSO or user-set                 │  ││
│  │  │  email:          From SSO or user-set                 │  ││
│  │  │  avatar_url:     From SSO or uploaded                 │  ││
│  │  │  roles:          Mapped from SSO groups               │  ││
│  │  │                                                       │  ││
│  │  └──────────────────────────────────────────────────────┘  ││
│  │                                                              ││
│  └──────────────────────────────────────────────────────────────┘│
│                                                                  │
│  SSO Attribute Mapping (konfigurierbar):                        │
│  ───────────────────────────────────────                        │
│  display_name:  preferred_username → name → email               │
│  avatar:        picture → avatar_url → (none)                   │
│  email:         email                                            │
│  groups:        groups → roles → (none)                         │
│                                                                  │
│  JIT Provisioning Flow:                                         │
│  ──────────────────────                                         │
│  1. User klickt "Login with SSO"                                │
│  2. Redirect zu OIDC Provider                                   │
│  3. User authentifiziert sich                                   │
│  4. Callback mit Authorization Code                             │
│  5. Token Exchange für ID Token                                 │
│  6. UserInfo abrufen                                            │
│  7. User existiert? → Session erstellen                         │
│     User neu? → Profil erstellen, dann Session                  │
│  8. Redirect zu App mit Session Cookie                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 6. Deployment Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                   DOCKER DEPLOYMENT                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  docker-compose.yml                                              │
│  ──────────────────                                              │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Docker Network                            ││
│  │                   (voicechat_net)                            ││
│  │                                                              ││
│  │  ┌──────────────┐                                           ││
│  │  │   Traefik    │ ◄─── Port 443 (HTTPS)                     ││
│  │  │  (Reverse    │ ◄─── Port 80 (HTTP → HTTPS Redirect)      ││
│  │  │   Proxy)     │                                           ││
│  │  └──────┬───────┘                                           ││
│  │         │                                                    ││
│  │         ├──────────────────┬─────────────────┐              ││
│  │         │                  │                 │              ││
│  │         ▼                  ▼                 ▼              ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      ││
│  │  │ voicechat-   │  │ voicechat-   │  │ voicechat-   │      ││
│  │  │ api          │  │ voice        │  │ web          │      ││
│  │  │              │  │              │  │ (optional)   │      ││
│  │  │ Auth + Chat  │  │ SFU Server   │  │ Static Files │      ││
│  │  │ Services     │  │ WebRTC       │  │              │      ││
│  │  └──────┬───────┘  └──────┬───────┘  └──────────────┘      ││
│  │         │                 │                                  ││
│  │         │                 │  UDP Ports: 10000-10100          ││
│  │         │                 │  (WebRTC Media)                  ││
│  │         │                 │                                  ││
│  │         ▼                 │                                  ││
│  │  ┌──────────────┐        │                                  ││
│  │  │   Redis      │◄───────┘                                  ││
│  │  │              │                                           ││
│  │  │ Sessions,    │                                           ││
│  │  │ Presence,    │                                           ││
│  │  │ Pub/Sub      │                                           ││
│  │  └──────────────┘                                           ││
│  │         │                                                    ││
│  │         ▼                                                    ││
│  │  ┌──────────────┐                                           ││
│  │  │  PostgreSQL  │                                           ││
│  │  │              │                                           ││
│  │  │ Persistent   │                                           ││
│  │  │ Data         │                                           ││
│  │  └──────────────┘                                           ││
│  │                                                              ││
│  └──────────────────────────────────────────────────────────────┘│
│                                                                  │
│  Volumes:                                                        │
│  ────────                                                        │
│  • postgres_data    - Datenbank-Persistenz                      │
│  • redis_data       - Redis Persistenz (optional)               │
│  • uploads          - Lokale Datei-Uploads (oder S3)            │
│  • certs            - TLS Zertifikate (wenn nicht Let's Encrypt)│
│                                                                  │
│  Externe Verbindungen:                                          │
│  ─────────────────────                                          │
│  • S3-kompatibles Storage (MinIO, Backblaze, AWS)               │
│  • SMTP Server (für E-Mail-Benachrichtigungen)                  │
│  • OIDC Provider (Authentik, Keycloak, etc.)                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 7. Backup & Recovery

```
┌─────────────────────────────────────────────────────────────────┐
│                    BACKUP ARCHITECTURE                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Backup-Komponenten:                                            │
│  ───────────────────                                            │
│                                                                  │
│  1. PostgreSQL Database                                         │
│     • pg_dump täglich um 03:00 UTC                              │
│     • WAL Archiving für Point-in-Time Recovery                  │
│     • Retention: 30 Tage                                        │
│                                                                  │
│  2. Uploaded Files                                              │
│     • S3 Sync/Versioning                                        │
│     • Oder: tar + encrypt bei lokalem Storage                   │
│                                                                  │
│  3. Configuration                                               │
│     • docker-compose.yml                                        │
│     • .env Dateien (verschlüsselt)                              │
│     • TLS Zertifikate                                           │
│                                                                  │
│  Backup-Flow:                                                   │
│  ────────────                                                   │
│                                                                  │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐       │
│  │  Cronjob    │────►│  Backup     │────►│  S3 Bucket  │       │
│  │  (03:00)    │     │  Script     │     │             │       │
│  └─────────────┘     └─────────────┘     └─────────────┘       │
│                             │                    │               │
│                             │              Lifecycle             │
│                             │              Policy                │
│                             │                    │               │
│                             ▼                    ▼               │
│                      ┌─────────────┐     ┌─────────────┐        │
│                      │   Encrypt   │     │   Delete    │        │
│                      │  AES-256    │     │   after     │        │
│                      │             │     │   30 days   │        │
│                      └─────────────┘     └─────────────┘        │
│                                                                  │
│  Restore-Prozess:                                               │
│  ────────────────                                               │
│                                                                  │
│  $ ./scripts/restore.sh --from s3://bucket/backup-2024-01-15    │
│                                                                  │
│  1. Services stoppen                                            │
│  2. Backup herunterladen + entschlüsseln                        │
│  3. PostgreSQL restore                                          │
│  4. Files restore                                               │
│  5. Services starten                                            │
│  6. Health Check                                                │
│                                                                  │
│  RTO (Recovery Time Objective): < 30 Minuten                    │
│  RPO (Recovery Point Objective): < 24 Stunden                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Technical Debt & Known Issues

*Last reviewed: 2026-01-08*

### Critical: N+1 Query in Message List Handler

**Location:** `server/src/chat/messages.rs:133-163`

The `list()` handler fetches author info individually for each message:

```rust
for msg in messages {
    let author = db::find_user_by_id(&state.db, msg.user_id).await?;
}
```

**Impact:** 100 messages = 101 database queries. This will cause performance issues at scale.

**Recommendation:** Add batch user lookup:
```rust
pub async fn find_users_by_ids(pool: &PgPool, ids: &[Uuid]) -> HashMap<Uuid, User>
```

**Priority:** Critical - fix before production

---

### Medium: Duplicated AuthorProfile Construction

**Location:** `server/src/chat/messages.rs` (lines 135-150, 212-227, 274-289)

Profile construction logic is repeated in `list`, `create`, and `update` handlers.

**Recommendation:** Add `From<User> for AuthorProfile` and `AuthorProfile::deleted(id)` factory.

**Priority:** High - reduces maintenance burden

---

### Medium: Status Serialization Fragility

**Location:** `server/src/chat/messages.rs:142`

```rust
status: format!("{:?}", u.status).to_lowercase()
```

Using Debug formatting for API output couples wire format to Rust internals.

**Recommendation:** Add explicit `UserStatus::as_str()` method.

**Priority:** Medium - breaking change if enum changes

---

### Low: Missing Channel Access Control

**Location:** `server/src/chat/messages.rs:117-126`

The `list` endpoint checks channel existence but not user membership.

**Status:** May be intentional for public community servers. Needs explicit documentation.

**Priority:** Low - document or implement based on requirements

---

### Low: No Rate Limiting

No rate limiting on API endpoints or WebSocket messages.

**Recommendation:** Add tower-governor for HTTP, per-user limits for WebSocket.

**Priority:** Medium for production - prevents abuse

---

## Persona Review Summary (2026-01-08)

### Elrond (Architecture)

**Verdict:** Fundamental approach is sound. Embedding author profiles in message responses is the right UX decision.

**Key Concerns:**
1. N+1 query pattern must be fixed before production
2. Consider introducing `UserProfileService` abstraction for presence features
3. API response shape should plan for future features (roles, badges)

### Faramir (Security)

**Verdict:** Auth middleware is correctly applied. JWT validation is sound.

**Key Concerns:**
1. Rate limiting is missing on all endpoints
2. Channel access control is not enforced
3. Add audit logging for sensitive operations
4. Consider token revocation mechanism

### Éowyn (Code Quality)

**Verdict:** Code is readable and follows project patterns.

**Key Concerns:**
1. DRY violation in AuthorProfile construction
2. Status serialization should use explicit method
3. Error types are well-structured with thiserror

### Legolas (Testing)

**Verdict:** No tests exist for message handlers.

**Key Concerns:**
1. Add integration tests for message CRUD
2. Test WebSocket broadcast on message events
3. Add performance tests for message list pagination

---

## Referenzen

- [PROJECT_SPEC.md](../project/specification.md) - Projektanforderungen
- [STANDARDS.md](../development/standards.md) - Verwendete Standards
- [LICENSE_COMPLIANCE.md](../ops/license-compliance.md) - Lizenzprüfung

# VoiceChat

A self-hosted voice and text chat platform for gaming communities.

[![CI](https://github.com/yourorg/voicechat/actions/workflows/ci.yml/badge.svg)](https://github.com/yourorg/voicechat/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Features

- **Low Latency Voice Chat** – WebRTC-based with Opus codec, optimized for gaming
- **End-to-End Encryption** – Text messages encrypted with Olm/Megolm
- **Self-Hosted** – Your data stays on your server
- **Lightweight Client** – Tauri-based desktop app with minimal resource usage
- **SSO Support** – Integrate with Authentik, Keycloak, Azure AD, and more
- **Open Source** – MIT/Apache-2.0 dual licensed

## Quick Start

### Server (Docker)

```bash
# Clone the repository
git clone https://github.com/yourorg/voicechat.git
cd voicechat

# Copy and edit environment file
cp .env.example .env
# Edit .env with your settings

# Start the server
cd infra/compose
docker compose up -d
```

### Desktop Client

Download the latest release from [Releases](https://github.com/yourorg/voicechat/releases).

## Development

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Docker & Docker Compose
- PostgreSQL 16 (or use Docker)
- Redis 7 (or use Docker)

### Setup

```bash
# Start development services
cd infra/compose
docker compose -f docker-compose.dev.yml up -d

# Run server
cd ../../server
cargo run

# Run client (in another terminal)
cd ../client
npm install
npm run tauri dev
```

## Project Structure

```
voicechat/
├── server/          # Backend server (Rust/Axum)
├── client/          # Desktop client (Tauri + Solid.js)
├── shared/          # Shared Rust libraries
│   ├── vc-common/   # Common types and protocols
│   └── vc-crypto/   # E2EE cryptography
├── infra/           # Infrastructure (Docker, scripts)
├── docs/            # Documentation
└── specs/           # Project specifications
```

## Documentation

- [Quick Start Guide](docs/setup/quick-start.md)
- [Configuration](docs/setup/configuration.md)
- [Architecture](specs/ARCHITECTURE.md)
- [API Reference](docs/api/)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

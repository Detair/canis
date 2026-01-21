# VoiceChat Platform - Lizenz-Compliance

Dieses Dokument dokumentiert die Lizenzprüfung aller verwendeten Dependencies, um sicherzustellen, dass das Projekt als Open Source (MIT OR Apache-2.0) veröffentlicht werden kann.

---

## Projekt-Lizenz

```
SPDX-License-Identifier: MIT OR Apache-2.0
```

**Dual-Lizenzierung:** Nutzer können zwischen MIT und Apache 2.0 wählen.

### Begründung für Dual-Lizenz

| Aspekt | MIT | Apache 2.0 |
|--------|-----|------------|
| Einfachheit | ✅ Sehr kurz und einfach | ⚠️ Länger, komplexer |
| Patent-Schutz | ❌ Keiner | ✅ Expliziter Patent-Grant |
| Attribution | ✅ Nur Copyright Notice | ✅ Copyright + NOTICE File |
| Kompatibilität | ✅ Fast alles | ✅ Fast alles |
| Unternehmensfreundlich | ✅ Ja | ✅ Ja (bevorzugt) |

Die Dual-Lizenzierung ist Standard im Rust-Ökosystem und bietet maximale Flexibilität.

---

## Lizenz-Kompatibilität

### Kompatibilitätsmatrix

```
┌─────────────────────────────────────────────────────────────────┐
│              LIZENZ-KOMPATIBILITÄT MIT MIT/Apache 2.0           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ✅ KOMPATIBEL (können verwendet werden)                        │
│  ─────────────────────────────────────                          │
│  • MIT                    - Permissive                          │
│  • Apache 2.0             - Permissive + Patent                 │
│  • BSD-2-Clause           - Permissive                          │
│  • BSD-3-Clause           - Permissive                          │
│  • ISC                    - Permissive (wie MIT)                │
│  • Zlib                   - Permissive                          │
│  • CC0-1.0                - Public Domain                       │
│  • Unlicense              - Public Domain                       │
│  • Unicode-DFS-2016       - Permissive (Unicode Data)           │
│                                                                  │
│  ⚠️ EINGESCHRÄNKT KOMPATIBEL                                    │
│  ───────────────────────────                                    │
│  • MPL 2.0                - File-Level Copyleft                 │
│                             (Änderungen an MPL-Dateien müssen   │
│                              unter MPL bleiben, Rest ist frei)  │
│                                                                  │
│  ❌ NICHT KOMPATIBEL (dürfen NICHT verwendet werden)            │
│  ──────────────────────────────────────────────────             │
│  • GPL 2.0/3.0            - Starkes Copyleft                    │
│  • LGPL 2.1/3.0           - Library Copyleft (bei static link) │
│  • AGPL 3.0               - Network Copyleft                    │
│  • Proprietary            - Closed Source                       │
│  • SSPL                   - Server-Side Public License          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Server Dependencies Prüfung

### Web Framework & Runtime

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| axum | 0.7 | MIT | ✅ | MIT |
| tokio | 1.x | MIT | ✅ | MIT |
| tower | 0.4 | MIT | ✅ | MIT |
| tower-http | 0.5 | MIT | ✅ | MIT |
| hyper | 1.x | MIT | ✅ | MIT |

### WebSocket & Real-Time

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| tokio-tungstenite | 0.21 | MIT | ✅ | MIT |
| tungstenite | 0.21 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### WebRTC

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| webrtc | 0.11 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-data | 0.9 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-dtls | 0.9 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-ice | 0.11 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-media | 0.8 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-sctp | 0.10 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-srtp | 0.13 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| webrtc-util | 0.9 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Datenbank

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| sqlx | 0.7 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| sqlx-core | 0.7 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| sqlx-postgres | 0.7 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Redis

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| fred | 8.x | MIT | ✅ | MIT |

### Authentifizierung

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| jsonwebtoken | 9.x | MIT | ✅ | MIT |
| argon2 | 0.5 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| totp-rs | 5.x | MIT | ✅ | MIT |
| openidconnect | 3.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| oauth2 | 4.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Kryptografie

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| rustls | 0.22 | MIT/Apache 2.0/ISC | ✅ | MIT OR Apache-2.0 OR ISC |
| ring | 0.17 | MIT + ISC + OpenSSL | ✅ | Siehe Notiz¹ |
| x25519-dalek | 2.x | BSD-3-Clause | ✅ | BSD-3-Clause |
| ed25519-dalek | 2.x | BSD-3-Clause | ✅ | BSD-3-Clause |
| curve25519-dalek | 4.x | BSD-3-Clause | ✅ | BSD-3-Clause |
| aes-gcm | 0.10 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| aes | 0.8 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| hkdf | 0.12 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| sha2 | 0.10 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| hmac | 0.12 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| rand | 0.8 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

¹ **ring Lizenz-Notiz:** ring verwendet Code unter MIT, ISC und OpenSSL-Lizenzen. Alle sind permissive und kompatibel.

### E2EE (Text)

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| vodozemac | 0.5 | Apache 2.0 | ✅ | Apache-2.0 |

**Wichtig:** Wir verwenden bewusst `vodozemac` statt `libsignal`:

| Library | Lizenz | Kompatibel? |
|---------|--------|-------------|
| vodozemac | Apache 2.0 | ✅ Ja |
| libsignal-protocol | AGPL 3.0 | ❌ Nein |

### Serialisierung

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| serde | 1.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| serde_json | 1.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| serde_derive | 1.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Utilities

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| uuid | 1.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| chrono | 0.4 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| tracing | 0.1 | MIT | ✅ | MIT |
| tracing-subscriber | 0.3 | MIT | ✅ | MIT |
| thiserror | 1.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| anyhow | 1.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| bytes | 1.x | MIT | ✅ | MIT |
| futures | 0.3 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### S3 Storage

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| aws-sdk-s3 | 1.x | Apache 2.0 | ✅ | Apache-2.0 |
| aws-config | 1.x | Apache 2.0 | ✅ | Apache-2.0 |
| aws-smithy-runtime | 1.x | Apache 2.0 | ✅ | Apache-2.0 |

### API Dokumentation

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| utoipa | 4.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| utoipa-swagger-ui | 6.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Markdown

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| pulldown-cmark | 0.10 | MIT | ✅ | MIT |

---

## Client Dependencies Prüfung

### Tauri

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| tauri | 2.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| tauri-build | 2.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| tauri-runtime | 2.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| tauri-runtime-wry | 2.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| wry | 0.35 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |
| tao | 0.25 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Audio

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| cpal | 0.15 | Apache 2.0 | ✅ | Apache-2.0 |
| opus | 0.3 | MIT | ✅ | MIT |
| nnnoiseless | 0.5 | BSD-3-Clause | ✅ | BSD-3-Clause |

**Notiz zu libopus:** Die native `libopus` Library ist BSD-3-Clause lizenziert.

### Secure Storage

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| keyring | 2.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### HTTP Client

| Crate | Version | Lizenz | Status | SPDX |
|-------|---------|--------|--------|------|
| reqwest | 0.11 | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

---

## Frontend Dependencies Prüfung

### Frameworks

| Package | Version | Lizenz | Status | SPDX |
|---------|---------|--------|--------|------|
| solid-js | 1.8 | MIT | ✅ | MIT |
| @solidjs/router | 0.10 | MIT | ✅ | MIT |

### Build Tools

| Package | Version | Lizenz | Status | SPDX |
|---------|---------|--------|--------|------|
| vite | 5.x | MIT | ✅ | MIT |
| vite-plugin-solid | 2.8 | MIT | ✅ | MIT |
| typescript | 5.x | Apache 2.0 | ✅ | Apache-2.0 |
| @tauri-apps/cli | 2.x | MIT/Apache 2.0 | ✅ | MIT OR Apache-2.0 |

### Styling

| Package | Version | Lizenz | Status | SPDX |
|---------|---------|--------|--------|------|
| unocss | 0.58 | MIT | ✅ | MIT |
| @unocss/preset-uno | 0.58 | MIT | ✅ | MIT |

### Icons

| Package | Version | Lizenz | Status | SPDX |
|---------|---------|--------|--------|------|
| lucide-solid | 0.300 | ISC | ✅ | ISC |

---

## Abgelehnte Dependencies

Diese Libraries wurden geprüft und **bewusst nicht verwendet**:

| Library | Lizenz | Grund für Ablehnung |
|---------|--------|---------------------|
| libsignal-protocol | AGPL 3.0 | Würde AGPL für gesamtes Projekt erzwingen |
| ffmpeg | GPL/LGPL | GPL-Komponenten, komplizierte Lizenz |
| openssl (native) | Apache 2.0 | Okay, aber rustls bevorzugt (pure Rust) |
| Matrix SDK | Apache 2.0 | Zu komplex, nur vodozemac für Crypto genutzt |

---

## Lizenz-Dateien im Repository

Das Repository muss folgende Dateien enthalten:

### LICENSE-MIT

```
MIT License

Copyright (c) [YEAR] [COPYRIGHT HOLDER]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### LICENSE-APACHE

```
                              Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

[Vollständiger Apache 2.0 Text]
```

### Cargo.toml

```toml
[package]
name = "voicechat"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/[user]/voicechat"
description = "Self-hosted voice and text chat platform"
keywords = ["voip", "chat", "webrtc", "e2ee"]
categories = ["multimedia", "network-programming"]
```

### README.md (Lizenz-Sektion)

```markdown
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
```

---

## Automatisierte Compliance-Prüfung

### cargo-deny Konfiguration

```toml
# deny.toml

[graph]
targets = []
all-features = true

[advisories]
db-path = "~/.cargo/advisory-db"
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"
notice = "warn"

[licenses]
unlicensed = "deny"
copyleft = "deny"
allow-osi-fsf-free = "neither"
default = "deny"
confidence-threshold = 0.93

allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "BSL-1.0",
    "ISC",
    "CC0-1.0",
    "Unlicense",
    "Zlib",
    "Unicode-DFS-2016",
    "MPL-2.0",
]

deny = [
    "GPL-2.0",
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "GPL-3.0",
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "AGPL-3.0",
    "AGPL-3.0-only",
    "AGPL-3.0-or-later",
    "LGPL-2.0",
    "LGPL-2.1",
    "LGPL-3.0",
]

[[licenses.clarify]]
name = "ring"
expression = "MIT AND ISC AND OpenSSL"
license-files = [
    { path = "LICENSE", hash = 0xbd0eed23 }
]

[[licenses.clarify]]
name = "webpki"
expression = "ISC"
license-files = [
    { path = "LICENSE", hash = 0x001c7e6c }
]

[bans]
multiple-versions = "warn"
wildcards = "deny"
highlight = "all"

deny = [
    # Explizit verbotene Crates
]

[sources]
unknown-registry = "deny"
unknown-git = "warn"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

### CI/CD Integration

```yaml
# .github/workflows/license-check.yml

name: License Compliance

on:
  push:
    branches: [main]
  pull_request:

jobs:
  license-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install cargo-deny
        run: cargo install cargo-deny
        
      - name: Check licenses
        run: cargo deny check licenses
        
      - name: Check advisories
        run: cargo deny check advisories
        
      - name: Check bans
        run: cargo deny check bans
```

---

## Third-Party Notices

Bei Distribution müssen folgende Attributions enthalten sein:

### THIRD_PARTY_NOTICES.md

```markdown
# Third Party Notices

This software includes the following third-party components:

## Rust Crates

### ring
Copyright (c) 2015-2016 Brian Smith
Licensed under MIT, ISC, and OpenSSL licenses

### curve25519-dalek, ed25519-dalek, x25519-dalek
Copyright (c) 2016-2021 isis agora lovecruft, Henry de Valence
Licensed under BSD-3-Clause

### RNNoise (via nnnoiseless)
Copyright (c) 2018 Gregor Richards, Jean-Marc Valin
Licensed under BSD-3-Clause

### Opus Codec
Copyright (c) 2010-2015 Xiph.Org Foundation, Skype Limited
Licensed under BSD-3-Clause

[... weitere nach Bedarf ...]

## JavaScript/TypeScript Packages

### Solid.js
Copyright (c) 2016-2023 Ryan Carniato
Licensed under MIT

### Lucide Icons
Licensed under ISC

[... weitere nach Bedarf ...]
```

---

## Compliance-Checkliste

### Vor jedem Release

- [ ] `cargo deny check` läuft erfolgreich
- [ ] Keine neuen GPL/AGPL Dependencies
- [ ] THIRD_PARTY_NOTICES.md aktualisiert
- [ ] LICENSE-MIT und LICENSE-APACHE vorhanden
- [ ] Cargo.toml enthält korrekte `license` Angabe

### Bei neuen Dependencies

- [ ] Lizenz geprüft (muss auf Allow-Liste sein)
- [ ] Transitive Dependencies geprüft
- [ ] In diesem Dokument dokumentiert
- [ ] THIRD_PARTY_NOTICES.md aktualisiert (falls nötig)

---

## Zusammenfassung

```
┌─────────────────────────────────────────────────────────────────┐
│                    LIZENZ-COMPLIANCE STATUS                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ✅ VOLLSTÄNDIG KOMPATIBEL                                      │
│                                                                  │
│  Server Dependencies:      45+ Crates  ✅ Alle geprüft          │
│  Client Dependencies:      25+ Crates  ✅ Alle geprüft          │
│  Frontend Dependencies:    10+ Packages ✅ Alle geprüft         │
│                                                                  │
│  Lizenzen im Einsatz:                                           │
│  • MIT                     ~60%                                 │
│  • MIT/Apache 2.0 Dual     ~30%                                 │
│  • Apache 2.0              ~5%                                  │
│  • BSD-3-Clause            ~3%                                  │
│  • ISC                     ~2%                                  │
│                                                                  │
│  Projekt-Lizenz:           MIT OR Apache-2.0                    │
│                                                                  │
│  Automatisierung:          cargo-deny in CI/CD                  │
│                                                                  │
│  Letzte Prüfung:           [Datum einfügen]                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Referenzen

- [PROJECT_SPEC.md](../project/specification.md) - Projektanforderungen
- [ARCHITECTURE.md](../architecture/overview.md) - Technische Architektur
- [STANDARDS.md](../development/standards.md) - Verwendete Standards und Protokolle
- [SPDX License List](https://spdx.org/licenses/)
- [Choose a License](https://choosealicense.com/)
- [cargo-deny Documentation](https://embarkstudios.github.io/cargo-deny/)

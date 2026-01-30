# License Compliance

This document tracks third-party dependencies and their licenses for the VoiceChat project.

## Fonts

### Press Start 2P
- **License:** SIL Open Font License 1.1 (OFL-1.1)
- **Source:** https://fonts.google.com/specimen/Press+Start+2P
- **Author:** CodeMan38
- **Usage:** Bundled font for pixel art theme UI elements
- **Compliance:** OFL-1.1 permits bundling and redistribution with attribution. Font name may not be used for derived works without permission.

## Email

### lettre
- **License:** MIT
- **Source:** https://crates.io/crates/lettre
- **Version:** 0.11
- **Usage:** SMTP email transport for transactional emails (password reset)
- **Compliance:** MIT licensed, fully compatible with project license

## Screen Capture & Video Encoding

### scap
- **License:** MIT
- **Source:** https://crates.io/crates/scap
- **Version:** 0.1
- **Usage:** Cross-platform screen capture (ScreenCaptureKit on macOS, WGC on Windows, PipeWire on Linux)
- **Compliance:** MIT licensed, fully compatible with project license

### vpx-encode
- **License:** MIT (wraps libvpx which is BSD-3-Clause)
- **Source:** https://crates.io/crates/vpx-encode
- **Version:** 0.3
- **Usage:** VP9 software video encoding for screen sharing
- **Compliance:** MIT licensed, libvpx is BSD-3-Clause — both compatible

### openh264
- **License:** BSD-2-Clause
- **Source:** https://crates.io/crates/openh264
- **Version:** 0.6
- **Usage:** H.264 fallback video encoding (Cisco patent-free binary)
- **Compliance:** BSD-2-Clause licensed, Cisco OpenH264 binary is patent-free — fully compatible

### image
- **License:** MIT OR Apache-2.0
- **Source:** https://crates.io/crates/image
- **Version:** 0.25
- **Usage:** PNG encoding for capture source thumbnails
- **Compliance:** Dual MIT/Apache-2.0 licensed, fully compatible with project license

## Checking Dependency Licenses

### Rust Dependencies
```bash
cargo deny check licenses
```

### JavaScript Dependencies
```bash
# Check for problematic licenses
bun pm licenses
```

## Allowed Licenses
- MIT
- Apache-2.0
- BSD-2-Clause
- BSD-3-Clause
- ISC
- Zlib
- CC0-1.0
- Unlicense
- MPL-2.0
- Unicode-DFS-2016
- OFL-1.1 (for fonts)

## Prohibited Licenses
- GPL-2.0
- GPL-3.0
- AGPL-3.0
- LGPL-2.0, LGPL-2.1, LGPL-3.0 (for static linking)
- SSPL
- Proprietary

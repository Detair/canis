<!-- Parent: ../AGENTS.md -->
# Tauri Capabilities

## Purpose

Tauri 2.0 capability configuration files. Defines security permissions and allowed APIs for the desktop application.

## Key Files

| File | Purpose |
|------|---------|
| `default.json` | Default capability set for the application window |

## For AI Agents

### What Are Capabilities?

Tauri 2.0 uses a capability-based security model:
- Each window has assigned capabilities
- Capabilities grant access to specific APIs
- Follows principle of least privilege

### Capability Structure

```json
{
  "$schema": "https://schemas.tauri.app/v2/capability.schema.json",
  "identifier": "default",
  "description": "Default capability for main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "http:default"
  ]
}
```

### Common Permissions

**Core:**
- `core:default` - Basic Tauri functionality
- `core:event:default` - Event system
- `core:window:default` - Window management

**Network:**
- `http:default` - HTTP requests
- `websocket:default` - WebSocket connections

**System:**
- `shell:allow-open` - Open URLs in default browser
- `notification:default` - System notifications
- `clipboard:default` - Clipboard access

### Adding New Permissions

1. Identify required Tauri plugin
2. Add permission to `permissions` array in `default.json`
3. Restart dev server for changes to take effect

### Security Considerations

**DO:**
- Request minimum necessary permissions
- Document why each permission is needed
- Review permissions during security audits

**DON'T:**
- Grant blanket `*:default` permissions
- Enable dangerous permissions without review
- Expose file system access unnecessarily

### Debugging Permissions

If a Tauri command fails with permission error:
1. Check console for specific permission required
2. Add to `default.json`
3. Verify it's the minimum required scope

### Plugin-Specific Capabilities

Some plugins have their own capability files. Check plugin documentation for required permissions.

## Dependencies

- Tauri 2.0 runtime
- Corresponds to plugins in `Cargo.toml` dependencies

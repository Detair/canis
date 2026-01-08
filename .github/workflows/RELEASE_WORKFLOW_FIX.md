# Release Workflow Fix - Priority 1 Issue

## Problem Fixed
The original release workflow created a GitHub Release but **didn't attach any build artifacts to it**. The Tauri builds were uploaded to GitHub Actions artifacts storage, not to the actual release.

## Solution Implemented

### Changed Architecture

**Before:**
```
create-release (deprecated actions/create-release@v1)
    ↓
build-tauri → upload to Actions artifacts (NOT attached to release)
    ↓
build-docker
```

**After:**
```
prepare-release (metadata extraction)
    ↓
    ├── build-tauri → upload to Actions artifacts
    └── build-docker → push to registry
            ↓
    publish-release (softprops/action-gh-release@v2)
        - Downloads all artifacts
        - Creates GitHub Release
        - Attaches all files to release
```

### Key Changes

1. **Replaced deprecated action**: Changed from `actions/create-release@v1` (deprecated) to `softprops/action-gh-release@v2` (modern, maintained)

2. **Three-stage pipeline**:
   - **Stage 1 (prepare-release)**: Extract version, generate changelog, detect prerelease
   - **Stage 2 (build-*)**: Build all artifacts and push Docker images
   - **Stage 3 (publish-release)**: Download artifacts and create GitHub Release with all files

3. **Proper artifact handling**:
   - `build-tauri` uploads to GitHub Actions artifacts (temporary storage)
   - `publish-release` downloads from artifacts and attaches to GitHub Release (permanent)

4. **Enhanced release body**: Added platform-specific installation instructions

## What Now Works

When you create a release (e.g., `git tag v0.1.0 && git push origin v0.1.0`):

1. Version and changelog are automatically generated
2. Tauri builds for all 6 targets:
   - Linux: `.deb` and `.AppImage`
   - Windows: `.msi` and `.exe` (NSIS installer)
   - macOS: `.dmg` for x86_64 and aarch64 (Apple Silicon)
3. Docker image pushed to `ghcr.io/<org>/canis/server:v0.1.0`
4. GitHub Release created with:
   - Auto-generated changelog from git history
   - All platform installers attached
   - Consolidated `checksums.txt` file
   - Installation instructions for each platform
   - Draft mode (requires manual publish)

## Testing

To test the workflow without publishing:

```bash
# Trigger manually with test tag
gh workflow run release.yml -f tag=v0.0.0-test
```

Check the Actions tab to see the pipeline run. The release will be created as a draft.

## Security Notes

- Uses `softprops/action-gh-release@v2` which is actively maintained
- All environment variables properly sanitized
- Release created as draft by default (requires manual approval before publishing)
- Prerelease detection: tags with "alpha", "beta", or "rc" marked as prerelease

## Future Improvements (Not Priority 1)

- Add GPG signing for artifacts
- Generate SBOM (Software Bill of Materials)
- Add verification that git tag matches `Cargo.toml` version
- Implement approval gate before making release public
- Add Slack/Discord notification on release completion

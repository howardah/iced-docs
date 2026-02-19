---
title: Distribution
description: Prepare release artifacts for desktop and web targets.
version: latest
last_updated: 2026-02-19
order: 6
---

# Distribution

Distribution strategy depends on your target platform, but keep one core app architecture.

## Native checklist

- Build with release profile: `cargo build --release`
- Verify window defaults (size/title/mode) before packaging
- Package executable with platform-specific installer tooling

## Web checklist

- Build static assets with your web pipeline (for example, Trunk)
- Validate asset paths and routing behavior
- Test in production-like hosting setup

## Pitfalls

- Target-specific code paths not guarded by `cfg`
- Missing fonts/icons/assets from final bundle
- Runtime settings only tested in debug profile

## Related

- [Reference: Widgets Overview](/latest/reference/widgets-overview)

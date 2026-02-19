---
title: Distribution
description: Prepare release artifacts for desktop and web targets.
version: latest
last_updated: 2026-02-19
order: 6
---

# Distribution

Distribution is mostly operational, but architecture choices in your Iced app (state shape, runtime setup, asset loading) strongly affect release stability.

## Use this when...

- You are preparing native installers or web deployment artifacts.
- You need a practical checklist before shipping.
- You want to reduce release-only regressions.

## Minimal example

```sh
cargo build --release
```

## How it works

Build and validate on each target you plan to support. Keep runtime configuration explicit so startup, theme, subscriptions, and window behavior are predictable in release builds.

## Common patterns

```sh
cargo build --release
cargo test
```

## Gotchas / tips

- Guard target-specific code with `cfg` checks.
- Verify assets (fonts/images/icons) in packaged output, not just dev runs.
- Run smoke tests for navigation, input, and async flows in release mode.

## Related

- [Bundling](/latest/guide/bundling)
- [Widgets Overview](/latest/reference/widgets-overview)

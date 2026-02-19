---
title: Bundling
description: Configure runtime behavior with iced application builder APIs.
version: latest
last_updated: 2026-02-19
order: 5
---

# Bundling

Bundling starts with stable runtime configuration. In Iced, that usually means switching to `iced::application(...)` and configuring behavior through builder methods.

## Use this when...

- You are moving from prototype to distributable app.
- You need explicit window/theme/subscription setup.
- You want startup behavior to be deterministic across environments.

## Minimal example

```rust
pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("My App")
        .window_size((1024.0, 720.0))
        .run()
}
```

## How it works

Bundling quality depends on consistent runtime config and asset handling. The builder API keeps these settings visible and testable.

## Common patterns

```rust
iced::application(App::new, App::update, App::view)
    .theme(App::theme)
    .subscription(App::subscription)
    .run()
```

## Gotchas / tips

- Test release builds early; debug behavior can hide timing/perf issues.
- Keep assets/fonts in known paths and verify they are included by packaging steps.
- Prefer explicit runtime config over scattered defaults.

## Related

- [Distribution](/latest/guide/distribution)
- [Runtime API](/latest/reference/runtime-api)

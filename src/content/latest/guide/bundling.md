---
title: Bundling
description: Configure runtime behavior with iced application builder APIs.
version: latest
last_updated: 2026-02-19
order: 5
---

# Bundling

Use the application builder when you need explicit runtime configuration.

## Verified API

`ref/doc/iced/fn.application.html` documents:

```rust
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>
```

## Common builder configuration

Official examples use methods like:

- `.title(...)`
- `.window_size((w, h))`
- `.subscription(...)`
- `.theme(...)`
- `.run()`

## Related

- [Distribution](/latest/guide/distribution)
- [Reference: Runtime API](/latest/reference/runtime-api)

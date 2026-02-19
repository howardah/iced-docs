---
title: Module - Text
description: Module-level reference for iced::widget::text.
version: latest
last_updated: 2026-02-19
order: 122
---

# Module - Text

Authoritative source: `ref/doc/iced/widget/text/index.html`.

## Rustdoc description

Draw and interact with text.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::text.

## Example References

- ref/examples/websocket/src/echo.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/changelog/src/icon.rs
- ref/examples/changelog/src/main.rs
- ref/examples/color_palette/src/main.rs


## Related

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need module-level APIs beyond the basic constructor call.
- You want family-specific style/state traits and helper types.
- You are building reusable widget abstractions.

## Minimal example

```rust
// Start with the constructor from this module family in `view`.
// Then move to module APIs for deeper customization.
```

## How it works

Module docs explain the namespace that groups constructors, types, and related traits. In everyday app code, this helps you discover advanced options after basic usage works.

## Common patterns

```rust
// Message flow pattern:
// widget interaction -> Message -> update -> state change -> rerender
```

## Gotchas / tips

- Check this page together with its family page for complete context.
- Verify trait bounds and associated types in rustdoc when custom styling fails.
- Keep module imports explicit while learning.

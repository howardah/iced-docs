---
title: Constructor - Bottom Right
description: Function reference for iced::widget::bottom_right.
version: latest
last_updated: 2026-02-19
order: 303
---

# Constructor - Bottom Right

Authoritative source: `ref/doc/iced/widget/fn.bottom_right.html`.

## Rustdoc summary

Creates a new
Container
that fills all the available space
and aligns its contents inside to the bottom right corner.

## Verified signature

```rust
pub fn bottom_right<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/custom_quad/src/main.rs

## Inline Examples (from rustdoc)

```rust
let bottom_right = container("Bottom!").align_right(Fill).align_bottom(Fill);
```

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want the canonical entrypoint for creating this widget/helper.
- You need concrete constructor arguments and builder chaining.
- You are wiring UI interactions into typed messages.

## Minimal example

```rust
// Call this constructor in `view`, then map events to Message variants.
```

## How it works

Constructors return typed widget values. You configure behavior via builder methods and emit `Message` values for `update` to handle.

## Common patterns

```rust
// Keep constructor calls close to their message mapping.
// Prefer small helper functions for repeated widget setups.
```

## Gotchas / tips

- Re-check argument order in the verified signature on this page.
- Keep side effects out of `view`; trigger them from `update` with Task.
- Use the related family page when deciding between module/element APIs.

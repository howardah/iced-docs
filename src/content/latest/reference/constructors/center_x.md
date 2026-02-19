---
title: Constructor - Center X
description: Function reference for iced::widget::center_x.
version: latest
last_updated: 2026-02-19
order: 306
---

# Constructor - Center X

Authoritative source: `ref/doc/iced/widget/fn.center_x.html`.

## Rustdoc summary

Creates a new
Container
that fills all the available space
horizontally and centers its contents inside.

## Verified signature

```rust
pub fn center_x<'a, Message, Theme, Renderer>(
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

- ref/examples/editor/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/geometry/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/progress_bar/src/main.rs
- ref/examples/multi_window/src/main.rs

## Inline Examples (from rustdoc)

```rust
let center_x = container("Horizontal Center!").center_x(Fill);
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

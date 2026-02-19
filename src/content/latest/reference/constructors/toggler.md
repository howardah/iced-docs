---
title: Constructor - Toggler
description: Function reference for iced::widget::toggler.
version: latest
last_updated: 2026-02-19
order: 341
---

# Constructor - Toggler

Authoritative source: ref/doc/iced/widget/fn.toggler.html.

## Rustdoc summary

Creates a new
Toggler
.

## Verified signature

```rust
pub fn toggler<'a, Message, Theme, Renderer>(
is_checked: bool,
) -> Toggler<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/styling/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/qr_code/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

---
title: Element - Toggler
description: Struct reference for iced::widget::Toggler.
version: latest
last_updated: 2026-02-19
order: 528
---

# Element - Toggler

Authoritative source: ref/doc/iced/widget/struct.Toggler.html.

## Rustdoc summary

A toggler widget.

## Verified type declaration

```rust
pub struct Toggler<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/tour/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/qr_code/src/main.rs
- ref/examples/styling/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

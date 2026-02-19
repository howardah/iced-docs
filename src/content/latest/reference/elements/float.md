---
title: Element - Float
description: Struct reference for iced::widget::Float.
version: latest
last_updated: 2026-02-19
order: 507
---

# Element - Float

Authoritative source: ref/doc/iced/widget/struct.Float.html.

## Rustdoc summary

A widget that can make its contents float over other widgets.

## Verified type declaration

```rust
pub struct Float<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/custom_shader/src/scene/pipeline.rs
- ref/examples/gallery/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

---
title: Element - Container
description: Struct reference for iced::widget::Container.
version: latest
last_updated: 2026-02-19
order: 506
---

# Element - Container

Authoritative source: ref/doc/iced/widget/struct.Container.html.

## Rustdoc summary

A widget that aligns its contents inside of its boundaries.

## Verified type declaration

```rust
pub struct Container<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>where
Theme: Catalog,
Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/gradient/src/main.rs
- ref/examples/tour/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

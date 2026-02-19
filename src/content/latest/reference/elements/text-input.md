---
title: Element - Text Input
description: Struct reference for iced::widget::TextInput.
version: latest
last_updated: 2026-02-19
order: 526
---

# Element - Text Input

Authoritative source: ref/doc/iced/widget/struct.TextInput.html.

## Rustdoc summary

A field that can be filled with text.

## Verified type declaration

```rust
pub struct TextInput<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
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
- ref/examples/changelog/src/main.rs
- ref/examples/integration/src/controls.rs
- ref/examples/lazy/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/multi_window/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

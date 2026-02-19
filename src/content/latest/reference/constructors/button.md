---
title: Constructor - Button
description: Function reference for iced::widget::button.
version: latest
last_updated: 2026-02-19
order: 304
---

# Constructor - Button

Authoritative source: `ref/doc/iced/widget/fn.button.html`.

## Rustdoc summary

Creates a new
Button
with the provided content.

## Verified signature

```rust
pub fn button<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/bezier_tool/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/events/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

---
title: Widget Constructor - radio
description: Function reference for iced::widget::radio.
version: latest
last_updated: 2026-02-19
order: 323
---

# Widget Constructor - iced::widget::radio

Authoritative source: ref/doc/iced/widget/fn.radio.html.

## Rustdoc summary

Creates a new
Radio
.

## Verified signature

```rust
pub fn radio<'a, Message, Theme, Renderer, V>(
label: impl Into<String>,
value: V,
selected: Option<V>,
on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, Theme, Renderer>where
Message: Clone,
Theme: Catalog + 'a,
Renderer: Renderer,
V: Copy + Eq,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/tour/src/main.rs
- ref/examples/scrollable/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors)
- [Widget Elements Catalog](/latest/reference/widget-elements)

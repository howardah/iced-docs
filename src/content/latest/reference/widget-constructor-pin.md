---
title: Widget Constructor - pin
description: Function reference for iced::widget::pin.
version: latest
last_updated: 2026-02-19
order: 321
---

# Widget Constructor - iced::widget::pin

Authoritative source: ref/doc/iced/widget/fn.pin.html.

## Rustdoc summary

Creates a new
Pin
widget with the given content.

## Verified signature

```rust
pub fn pin<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Pin<'a, Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/layout/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

---
title: Widget Constructor - float
description: Function reference for iced::widget::float.
version: latest
last_updated: 2026-02-19
order: 312
---

# Widget Constructor - iced::widget::float

Authoritative source: ref/doc/iced/widget/fn.float.html.

## Rustdoc summary

Creates a new
Float
widget with the given content.

## Verified signature

```rust
pub fn float<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Float<'a, Message, Theme, Renderer>where
Theme: Catalog,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/gallery/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

---
title: Widget Constructor - text
description: Function reference for iced::widget::text.
version: latest
last_updated: 2026-02-19
order: 337
---

# Widget Constructor - iced::widget::text

Authoritative source: ref/doc/iced/widget/fn.text.html.

## Rustdoc summary

Creates a new
Text
widget with the provided content.

## Verified signature

```rust
pub fn text<'a, Theme, Renderer>(
text: impl IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/layout/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/integration/src/controls.rs
- ref/examples/markdown/src/main.rs
- ref/examples/color_palette/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

---
title: Widget Constructor - responsive
description: Function reference for iced::widget::responsive.
version: latest
last_updated: 2026-02-19
order: 324
---

# Widget Constructor - iced::widget::responsive

Authoritative source: ref/doc/iced/widget/fn.responsive.html.

## Rustdoc summary

Creates a new
Responsive
widget with a closure that produces its
contents.

## Verified signature

```rust
pub fn responsive<'a, Message, Theme, Renderer>(
f: impl Fn(Size) -> Element<'a, Message, Theme, Renderer> + 'a,
) -> Responsive<'a, Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/layout/src/main.rs
- ref/examples/pane_grid/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors)
- [Widget Elements Catalog](/latest/reference/widget-elements)

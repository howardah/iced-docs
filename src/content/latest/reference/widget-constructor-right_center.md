---
title: Widget Constructor - right_center
description: Function reference for iced::widget::right_center.
version: latest
last_updated: 2026-02-19
order: 327
---

# Widget Constructor - iced::widget::right_center

Authoritative source: ref/doc/iced/widget/fn.right_center.html.

## Rustdoc summary

Creates a new
Container
that fills all the available space
and aligns its contents inside to the right center.

## Verified signature

```rust
pub fn right_center<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors)
- [Widget Elements Catalog](/latest/reference/widget-elements)

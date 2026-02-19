---
title: Widget Constructor - themer
description: Function reference for iced::widget::themer.
version: latest
last_updated: 2026-02-19
order: 340
---

# Widget Constructor - iced::widget::themer

Authoritative source: ref/doc/iced/widget/fn.themer.html.

## Rustdoc summary

A widget that applies any Theme to its contents.

## Verified signature

```rust
pub fn themer<'a, Message, Theme, Renderer>(
theme: Option<Theme>,
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Themer<'a, Message, Theme, Renderer>where
Theme: Base,
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

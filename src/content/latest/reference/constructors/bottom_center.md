---
title: Constructor - Bottom Center
description: Function reference for iced::widget::bottom_center.
version: latest
last_updated: 2026-02-19
order: 302
---

# Constructor - Bottom Center

Authoritative source: `ref/doc/iced/widget/fn.bottom_center.html`.

## Rustdoc summary

Creates a new
Container
that fills all the available space
and aligns its contents inside to the bottom center.

## Verified signature

```rust
pub fn bottom_center<'a, Message, Theme, Renderer>(
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

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

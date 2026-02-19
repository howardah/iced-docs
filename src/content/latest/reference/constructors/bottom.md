---
title: Constructor - Bottom
description: Function reference for iced::widget::bottom.
version: latest
last_updated: 2026-02-19
order: 301
---

# Constructor - Bottom

Authoritative source: ref/doc/iced/widget/fn.bottom.html.

## Rustdoc summary

Creates a new
Container
that fills all the available space
vertically and bottom-aligns its contents inside.

## Verified signature

```rust
pub fn bottom<'a, Message, Theme, Renderer>(
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

- ref/examples/integration/src/controls.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

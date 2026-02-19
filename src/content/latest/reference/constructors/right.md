---
title: Constructor - Right
description: Function reference for iced::widget::right.
version: latest
last_updated: 2026-02-19
order: 326
---

# Constructor - Right

Authoritative source: ref/doc/iced/widget/fn.right.html.

## Rustdoc summary

Creates a new
Container
that fills all the available space
horizontally and right-aligns its contents inside.

## Verified signature

```rust
pub fn right<'a, Message, Theme, Renderer>(
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

- ref/examples/markdown/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/text/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

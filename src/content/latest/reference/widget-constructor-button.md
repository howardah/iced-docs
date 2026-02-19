---
title: Widget Constructor - button
description: Function reference for iced::widget::button.
version: latest
last_updated: 2026-02-19
order: 304
---

# Widget Constructor - iced::widget::button

Authoritative source: ref/doc/iced/widget/fn.button.html.

## Rustdoc summary

Creates a new
Button
with the provided content.

## Verified signature

```rust
pub fn button<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/events/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/game_of_life/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

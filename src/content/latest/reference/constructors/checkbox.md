---
title: Constructor - Checkbox
description: Function reference for iced::widget::checkbox.
version: latest
last_updated: 2026-02-19
order: 308
---

# Constructor - Checkbox

Authoritative source: ref/doc/iced/widget/fn.checkbox.html.

## Rustdoc summary

Creates a new
Checkbox
.

## Verified signature

```rust
pub fn checkbox<'a, Message, Theme, Renderer>(
is_checked: bool,
) -> Checkbox<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/todos/src/main.rs
- ref/examples/events/src/main.rs
- ref/examples/vectorial_text/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/progress_bar/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

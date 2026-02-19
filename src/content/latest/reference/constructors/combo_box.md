---
title: Constructor - Combo Box
description: Function reference for iced::widget::combo_box.
version: latest
last_updated: 2026-02-19
order: 310
---

# Constructor - Combo Box

Authoritative source: `ref/doc/iced/widget/fn.combo_box.html`.

## Rustdoc summary

Creates a new
ComboBox
.

## Verified signature

```rust
pub fn combo_box<'a, T, Message, Theme, Renderer>(
    state: &'a State<T>,
    placeholder: &str,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<'a, T, Message, Theme, Renderer>
where
    T: Display + Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/combo_box/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

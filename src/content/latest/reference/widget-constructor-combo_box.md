---
title: Widget Constructor - combo_box
description: Function reference for iced::widget::combo_box.
version: latest
last_updated: 2026-02-19
order: 310
---

# Widget Constructor - iced::widget::combo_box

Authoritative source: ref/doc/iced/widget/fn.combo_box.html.

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
) -> ComboBox<'a, T, Message, Theme, Renderer>where
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

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

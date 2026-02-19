---
title: Element - Combo Box
description: Struct reference for iced::widget::ComboBox.
version: latest
last_updated: 2026-02-19
order: 505
---

# Element - Combo Box

Authoritative source: ref/doc/iced/widget/struct.ComboBox.html.

## Rustdoc summary

A widget for searching and selecting a single value from a list of options.

## Verified type declaration

```rust
pub struct ComboBox<'a, T, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/combo_box/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

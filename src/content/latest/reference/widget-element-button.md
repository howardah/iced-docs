---
title: Widget Element - Button
description: Struct reference for iced::widget::Button.
version: latest
last_updated: 2026-02-19
order: 502
---

# Widget Element - iced::widget::Button

Authoritative source: ref/doc/iced/widget/struct.Button.html.

## Rustdoc summary

A generic widget that produces a message when pressed.

## Verified type declaration

```rust
pub struct Button<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>where
Renderer: Renderer,
Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/tour/src/main.rs
- ref/examples/pokedex/src/main.rs

## Related

- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)

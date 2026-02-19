---
title: Constructor - Iced
description: Function reference for iced::widget::iced.
version: latest
last_updated: 2026-02-19
order: 315
---

# Constructor - Iced

Authoritative source: `ref/doc/iced/widget/fn.iced.html`.

## Rustdoc summary

Creates an
Element
that displays the iced logo with the given text_size.

## Verified signature

```rust
pub fn iced<'a, Message, Theme, Renderer>(
    text_size: impl Into<Pixels>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: Renderer + Renderer<Font = Font> + 'a,
    Theme: Catalog + Catalog + 'a,
    <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
    <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/slider/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

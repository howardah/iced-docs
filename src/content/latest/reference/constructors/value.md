---
title: Constructor - Value
description: Function reference for iced::widget::value.
version: latest
last_updated: 2026-02-19
order: 343
---

# Constructor - Value

Authoritative source: ref/doc/iced/widget/fn.value.html.

## Rustdoc summary

Creates a new
Text
widget that displays the provided value.

## Verified signature

```rust
pub fn value<'a, Theme, Renderer>(
value: impl ToString,
) -> Text<'a, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/gallery/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

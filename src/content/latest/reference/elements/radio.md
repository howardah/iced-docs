---
title: Element - Radio
description: Struct reference for iced::widget::Radio.
version: latest
last_updated: 2026-02-19
order: 515
---

# Element - Radio

Authoritative source: ref/doc/iced/widget/struct.Radio.html.

## Rustdoc summary

A circular button representing a choice.

## Verified type declaration

```rust
pub struct Radio<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer, {/* private fields */}
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/scrollable/src/main.rs
- ref/examples/tour/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

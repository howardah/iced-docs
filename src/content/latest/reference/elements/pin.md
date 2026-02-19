---
title: Element - Pin
description: Struct reference for iced::widget::Pin.
version: latest
last_updated: 2026-02-19
order: 513
---

# Element - Pin

Authoritative source: ref/doc/iced/widget/struct.Pin.html.

## Rustdoc summary

A widget that positions its contents at some fixed coordinates inside of its boundaries.

## Verified type declaration

```rust
pub struct Pin<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer, {/* private fields */}
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/layout/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

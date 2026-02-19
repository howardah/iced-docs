---
title: Element - Button
description: Struct reference for iced::widget::Button.
version: latest
last_updated: 2026-02-19
order: 502
---

# Element - Button

Authoritative source: ref/doc/iced/widget/struct.Button.html.

## Rustdoc summary

A generic widget that produces a message when pressed.

## Verified type declaration

```rust
/private/var/folders/hk/wykb6d7d2m5cg1rjhfx_tw200000gn/T/tmp.gDTBKDagjh:

pub struct Button<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,
    Theme: Catalog, {/* private fields */}
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/bezier_tool/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/events/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

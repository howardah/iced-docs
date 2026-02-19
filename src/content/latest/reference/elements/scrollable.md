---
title: Element - Scrollable
description: Struct reference for iced::widget::Scrollable.
version: latest
last_updated: 2026-02-19
order: 519
---

# Element - Scrollable

Authoritative source: ref/doc/iced/widget/struct.Scrollable.html.

## Rustdoc summary

A widget that can vertically display an infinite amount of content with a
scrollbar.

## Verified type declaration

```rust
pub struct Scrollable<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
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
- ref/examples/changelog/src/main.rs
- ref/examples/combo_box/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/geometry/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

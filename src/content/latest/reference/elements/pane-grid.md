---
title: Element - Pane Grid
description: Struct reference for iced::widget::PaneGrid.
version: latest
last_updated: 2026-02-19
order: 511
---

# Element - Pane Grid

Authoritative source: ref/doc/iced/widget/struct.PaneGrid.html.

## Rustdoc summary

A collection of panes distributed using either vertical or horizontal splits
to completely fill the space available.

## Verified type declaration

```rust
pub struct PaneGrid<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/pane_grid/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

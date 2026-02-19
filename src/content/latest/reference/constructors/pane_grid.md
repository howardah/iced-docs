---
title: Constructor - Pane Grid
description: Function reference for iced::widget::pane_grid.
version: latest
last_updated: 2026-02-19
order: 319
---

# Constructor - Pane Grid

Authoritative source: `ref/doc/iced/widget/fn.pane_grid.html`.

## Rustdoc summary

Creates a
PaneGrid
with the given
pane_grid::State
and view function.

## Verified signature

```rust
pub fn pane_grid<'a, T, Message, Theme, Renderer>(
state: &'a State<T>,
view: impl Fn(Pane, &'a T, bool) -> Content<'a, Message, Theme, Renderer>,
) -> PaneGrid<'a, Message, Theme, Renderer>where
Theme: Catalog,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/pane_grid/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

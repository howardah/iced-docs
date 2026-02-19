---
title: Family - Center
description: Unified reference for the Center widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 706
---

# Family - Center

This page unifies related iced::widget APIs for the **Center** family.

## API surfaces

- Constructor: [iced::widget::center](/latest/reference/constructors/center)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
and centers its contents inside.

## Verified constructor signature

```rust
pub fn center<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/solar_system/src/main.rs
- ref/examples/text/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let center = container("Center!").center(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

---
title: Family - Iced
description: Unified reference for the Iced widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 716
---

# Family - Iced

This page unifies related iced::widget APIs for the **Iced** family.

## API surfaces

- Constructor: [iced::widget::iced](/latest/reference/constructors/iced)

## Surface summaries

### Constructor

Creates an
Element
that displays the iced logo with the given text_size.

## Verified constructor signature

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
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/geometry/src/main.rs
- ref/examples/pokedex/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

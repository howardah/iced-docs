---
title: Family - Value
description: Unified reference for the Value widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 750
---

# Family - Value

This page unifies related iced::widget APIs for the **Value** family.

## API surfaces

- Constructor: [iced::widget::value](/latest/reference/constructors/value)

## Surface summaries

### Constructor

Creates a new
Text
widget that displays the provided value.

## Verified constructor signature

```rust
pub fn value<'a, Theme, Renderer>(
    value: impl ToString,
) -> Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/svg/src/main.rs
- ref/examples/loupe/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

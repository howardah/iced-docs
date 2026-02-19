---
title: Family - Keyed Column
description: Unified reference for the Keyed Column widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 719
---

# Family - Keyed Column

This page unifies related iced::widget APIs for the **Keyed Column** family.

## API surfaces

- Constructor: [iced::widget::keyed_column](/latest/reference/constructors/keyed_column)

## Surface summaries

### Constructor

Creates a new
keyed::Column
from an iterator of elements.

## Verified constructor signature

```rust
pub fn keyed_column<'a, Key, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = (Key, Element<'a, Message, Theme, Renderer>)>,
) -> Column<'a, Key, Message, Theme, Renderer>
where
    Key: Copy + PartialEq,
    Renderer: Renderer,
```
## Example References

- ref/examples/loupe/src/main.rs
- ref/examples/clock/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/arc/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::{keyed_column, text};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    keyed_column((0..=100).map(|i| {
        (i, text!("Item {i}").into())
    })).into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

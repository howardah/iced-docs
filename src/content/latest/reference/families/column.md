---
title: Family - Column
description: Unified reference for the Column widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 710
---

# Family - Column

This page unifies related iced::widget APIs for the **Column** family.

## API surfaces

- Constructor: [iced::widget::column](/latest/reference/constructors/column)
- Element: [iced::widget::Column](/latest/reference/elements/column)

## Surface summaries

### Constructor

Creates a new
Column
with the given children.

### Element

Creates a
Grid
with the given children.

## Verified constructor signature

```rust
pub fn column<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Column<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Column<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/tour/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/lazy/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/layout/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::{column, text};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    column((0..5).map(|i| text!("Item {i}").into())).into()
}
```

### Element example

```rust
use iced::widget::{button, column};

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    column![
        "I am on top!",
        button("I am in the center!"),
        "I am below.",
    ].into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

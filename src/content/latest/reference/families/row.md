---
title: Family - Row
description: Unified reference for the Row widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 733
---

# Family - Row

This page unifies related iced::widget APIs for the **Row** family.

## API surfaces

- Module: [iced::widget::row](/latest/reference/modules/row)
- Constructor: [iced::widget::row](/latest/reference/constructors/row)
- Element: [iced::widget::Row](/latest/reference/elements/row)

## Surface summaries

### Module

Distribute content horizontally.

### Constructor

Creates a new
Row
from an iterator.

### Element

Creates a
Stack
with the given children.

## Verified constructor signature

```rust
pub fn row<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Row<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Row<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/game_of_life/src/preset.rs
- ref/examples/layout/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/tour/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::{row, text};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    row((0..5).map(|i| text!("Item {i}").into())).into()
}
```

### Element example

```rust
use iced::widget::{button, row};

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    row![
        "I am to the left!",
        button("I am in the middle!"),
        "I am to the right!",
    ].into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want one page that links module, constructor, and element surfaces.
- You are deciding which API surface to start from.
- You need a practical map for this widget domain.

## Minimal example

```rust
// Typical flow:
// 1) Start with constructor usage.
// 2) Move to module docs for style/state details.
// 3) Use element docs for type-level control.
```

## How it works

Family pages connect related docs so you do not miss capabilities that are split across constructor/module/element pages.

## Common patterns

```rust
// Build with constructor APIs first,
// then refine behavior/styles through related module and element docs.
```

## Gotchas / tips

- Family routes normalize naming; module/function/struct names may differ slightly.
- Prefer this page as your entrypoint when learning unfamiliar widgets.
- Follow example references here before inventing integration patterns.

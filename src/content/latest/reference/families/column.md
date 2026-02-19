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
- ref/examples/layout/src/main.rs
- ref/examples/events/src/main.rs
- ref/examples/lazy/src/main.rs
- ref/examples/websocket/src/main.rs
- ref/examples/download_progress/src/main.rs

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

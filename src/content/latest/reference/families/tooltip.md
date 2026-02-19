---
title: Family - Tooltip
description: Unified reference for the Tooltip widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 749
---

# Family - Tooltip

This page unifies related iced::widget APIs for the **Tooltip** family.

## API surfaces

- Module: [iced::widget::tooltip](/latest/reference/modules/tooltip)
- Constructor: [iced::widget::tooltip](/latest/reference/constructors/tooltip)
- Element: [iced::widget::Tooltip](/latest/reference/elements/tooltip)

## Surface summaries

### Module

Tooltips display a hint of information over some element when hovered.

### Constructor

Creates a new
Tooltip
for the provided content with the given

Element
and
tooltip::Position
.

### Element

An element to display a widget over another.

## Verified constructor signature

```rust
pub fn tooltip<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    tooltip: impl Into<Element<'a, Message, Theme, Renderer>>,
    position: Position,
) -> Tooltip<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Tooltip<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/editor/src/main.rs
- ref/examples/tooltip/src/main.rs
- ref/examples/table/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::{container, tooltip};

enum Message {
    // ...
}

fn view(_state: &State) -> Element<'_, Message> {
    tooltip(
        "Hover me to display the tooltip!",
        container("This is the tooltip contents!")
            .padding(10)
            .style(container::rounded_box),
        tooltip::Position::Bottom,
    ).into()
}
```

### Element example

```rust
use iced::widget::{container, tooltip};

enum Message {
    // ...
}

fn view(_state: &State) -> Element<'_, Message> {
    tooltip(
        "Hover me to display the tooltip!",
        container("This is the tooltip contents!")
            .padding(10)
            .style(container::rounded_box),
        tooltip::Position::Bottom,
    ).into()
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

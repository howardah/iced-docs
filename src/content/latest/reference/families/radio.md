---
title: Family - Radio
description: Unified reference for the Radio widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 728
---

# Family - Radio

This page unifies related iced::widget APIs for the **Radio** family.

## API surfaces

- Module: [iced::widget::radio](/latest/reference/modules/radio)
- Constructor: [iced::widget::radio](/latest/reference/constructors/radio)
- Element: [iced::widget::Radio](/latest/reference/elements/radio)

## Surface summaries

### Module

Radio buttons let users choose a single option from a bunch of options.

### Constructor

Creates a new
Radio
.

### Element

A circular button representing a choice.

## Verified constructor signature

```rust
pub fn radio<'a, Message, Theme, Renderer, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
    V: Copy + Eq,
```

## Verified element declaration

```rust
pub struct Radio<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/tour/src/main.rs
- ref/examples/scrollable/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::{column, radio};

struct State {
   selection: Option<Choice>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(Choice),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    A,
    B,
    C,
    All,
}

fn view(state: &State) -> Element<'_, Message> {
    let a = radio(
        "A",
        Choice::A,
        state.selection,
        Message::RadioSelected,
    );

    let b = radio(
        "B",
        Choice::B,
        state.selection,
        Message::RadioSelected,
    );

    let c = radio(
        "C",
        Choice::C,
        state.selection,
        Message::RadioSelected,
    );

    let all = radio(
        "All of the above",
        Choice::All,
        state.selection,
        Message::RadioSelected
    );

    column![a, b, c, all].into()
}
```

### Element example

```rust
use iced::widget::{column, radio};

struct State {
   selection: Option<Choice>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(Choice),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    A,
    B,
    C,
    All,
}

fn view(state: &State) -> Element<'_, Message> {
    let a = radio(
        "A",
        Choice::A,
        state.selection,
        Message::RadioSelected,
    );

    let b = radio(
        "B",
        Choice::B,
        state.selection,
        Message::RadioSelected,
    );

    let c = radio(
        "C",
        Choice::C,
        state.selection,
        Message::RadioSelected,
    );

    let all = radio(
        "All of the above",
        Choice::All,
        state.selection,
        Message::RadioSelected
    );

    column![a, b, c, all].into()
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

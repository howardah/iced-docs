---
title: Family - Checkbox
description: Unified reference for the Checkbox widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 709
---

# Family - Checkbox

This page unifies related iced::widget APIs for the **Checkbox** family.

## API surfaces

- Module: [iced::widget::checkbox](/latest/reference/modules/checkbox)
- Constructor: [iced::widget::checkbox](/latest/reference/constructors/checkbox)
- Element: [iced::widget::Checkbox](/latest/reference/elements/checkbox)

## Surface summaries

### Module

Checkboxes can be used to let users make binary choices.

### Constructor

Creates a new
Checkbox
.

### Element

A box that can be checked.

## Verified constructor signature

```rust
pub fn checkbox<'a, Message, Theme, Renderer>(
    is_checked: bool,
) -> Checkbox<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Checkbox<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/svg/src/main.rs
- ref/examples/layout/src/main.rs
- ref/examples/checkbox/src/main.rs
- ref/examples/tooltip/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/game_of_life/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::checkbox;

struct State {
   is_checked: bool,
}

enum Message {
    CheckboxToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    checkbox(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::CheckboxToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::CheckboxToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

### Element example

```rust
use iced::widget::checkbox;

struct State {
   is_checked: bool,
}

enum Message {
    CheckboxToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    checkbox(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::CheckboxToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::CheckboxToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
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

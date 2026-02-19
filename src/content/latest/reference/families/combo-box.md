---
title: Family - Combo Box
description: Unified reference for the Combo Box widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 711
---

# Family - Combo Box

This page unifies related iced::widget APIs for the **Combo Box** family.

## API surfaces

- Module: [iced::widget::combo_box](/latest/reference/modules/combo_box)
- Constructor: [iced::widget::combo_box](/latest/reference/constructors/combo_box)
- Element: [iced::widget::ComboBox](/latest/reference/elements/combo-box)

## Surface summaries

### Module

Combo boxes display a dropdown list of searchable and selectable options.

### Constructor

Creates a new
ComboBox
.

### Element

A widget for searching and selecting a single value from a list of options.

## Verified constructor signature

```rust
pub fn combo_box<'a, T, Message, Theme, Renderer>(
    state: &'a State<T>,
    placeholder: &str,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<'a, T, Message, Theme, Renderer>
where
    T: Display + Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct ComboBox<'a, T, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/combo_box/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::combo_box;

struct State {
   fruits: combo_box::State<Fruit>,
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
}

#[derive(Debug, Clone)]
enum Message {
    FruitSelected(Fruit),
}

fn view(state: &State) -> Element<'_, Message> {
    combo_box(
        &state.fruits,
        "Select your favorite fruit...",
        state.favorite.as_ref(),
        Message::FruitSelected
    )
    .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FruitSelected(fruit) => {
            state.favorite = Some(fruit);
        }
    }
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Orange => "Orange",
            Self::Strawberry => "Strawberry",
            Self::Tomato => "Tomato",
        })
    }
}
```

### Element example

```rust
use iced::widget::combo_box;

struct State {
   fruits: combo_box::State<Fruit>,
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
}

#[derive(Debug, Clone)]
enum Message {
    FruitSelected(Fruit),
}

fn view(state: &State) -> Element<'_, Message> {
    combo_box(
        &state.fruits,
        "Select your favorite fruit...",
        state.favorite.as_ref(),
        Message::FruitSelected
    )
    .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FruitSelected(fruit) => {
            state.favorite = Some(fruit);
        }
    }
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Orange => "Orange",
            Self::Strawberry => "Strawberry",
            Self::Tomato => "Tomato",
        })
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

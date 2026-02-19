---
title: Family - Pick List
description: Unified reference for the Pick List widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 725
---

# Family - Pick List

This page unifies related iced::widget APIs for the **Pick List** family.

## API surfaces

- Module: [iced::widget::pick_list](/latest/reference/modules/pick_list)
- Constructor: [iced::widget::pick_list](/latest/reference/constructors/pick_list)
- Element: [iced::widget::PickList](/latest/reference/elements/pick-list)

## Surface summaries

### Module

Pick lists display a dropdown list of selectable options.

### Constructor

Creates a new
PickList
.

### Element

A widget for selecting a single value from a list of options.

## Verified constructor signature

```rust
pub fn pick_list<'a, T, L, V, Message, Theme, Renderer>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: Catalog + Catalog,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct PickList<'a, T, L, V, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/lazy/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/pick_list/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/qr_code/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::pick_list;

struct State {
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    let fruits = [
        Fruit::Apple,
        Fruit::Orange,
        Fruit::Strawberry,
        Fruit::Tomato,
    ];

    pick_list(
        fruits,
        state.favorite,
        Message::FruitSelected,
    )
    .placeholder("Select your favorite fruit...")
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
use iced::widget::pick_list;

struct State {
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    let fruits = [
        Fruit::Apple,
        Fruit::Orange,
        Fruit::Strawberry,
        Fruit::Tomato,
    ];

    pick_list(
        fruits,
        state.favorite,
        Message::FruitSelected,
    )
    .placeholder("Select your favorite fruit...")
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

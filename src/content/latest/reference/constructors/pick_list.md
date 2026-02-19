---
title: Constructor - Pick List
description: Function reference for iced::widget::pick_list.
version: latest
last_updated: 2026-02-19
order: 320
---

# Constructor - Pick List

Authoritative source: `ref/doc/iced/widget/fn.pick_list.html`.

## Rustdoc summary

Creates a new
PickList
.

## Verified signature

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

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/lazy/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/pick_list/src/main.rs
- ref/examples/toast/src/main.rs

## Inline Examples (from rustdoc)

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

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

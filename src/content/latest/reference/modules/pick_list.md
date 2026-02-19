---
title: Module - Pick List
description: Module-level reference for iced::widget::pick_list.
version: latest
last_updated: 2026-02-19
order: 111
---

# Module - Pick List

Authoritative source: `ref/doc/iced/widget/pick_list/index.html`.

## Rustdoc description

Pick lists display a dropdown list of selectable options.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::pick_list.

## Example References

- ref/examples/pick_list/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/layout/src/main.rs

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

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need module-level APIs beyond the basic constructor call.
- You want family-specific style/state traits and helper types.
- You are building reusable widget abstractions.

## Minimal example

```rust
// Start with the constructor from this module family in `view`.
// Then move to module APIs for deeper customization.
```

## How it works

Module docs explain the namespace that groups constructors, types, and related traits. In everyday app code, this helps you discover advanced options after basic usage works.

## Common patterns

```rust
// Message flow pattern:
// widget interaction -> Message -> update -> state change -> rerender
```

## Gotchas / tips

- Check this page together with its family page for complete context.
- Verify trait bounds and associated types in rustdoc when custom styling fails.
- Keep module imports explicit while learning.

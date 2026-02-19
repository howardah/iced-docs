---
title: Module - Combo Box
description: Module-level reference for iced::widget::combo_box.
version: latest
last_updated: 2026-02-19
order: 103
---

# Module - Combo Box

Authoritative source: `ref/doc/iced/widget/combo_box/index.html`.

## Rustdoc description

Combo boxes display a dropdown list of searchable and selectable options.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::combo_box.

## Example References

- ref/examples/combo_box/src/main.rs

## Inline Examples (from rustdoc)

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

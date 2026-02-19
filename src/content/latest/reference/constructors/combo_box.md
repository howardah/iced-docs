---
title: Constructor - Combo Box
description: Function reference for iced::widget::combo_box.
version: latest
last_updated: 2026-02-19
order: 310
---

# Constructor - Combo Box

Authoritative source: `ref/doc/iced/widget/fn.combo_box.html`.

## Rustdoc summary

Creates a new
ComboBox
.

## Verified signature

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

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

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

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want the canonical entrypoint for creating this widget/helper.
- You need concrete constructor arguments and builder chaining.
- You are wiring UI interactions into typed messages.

## Minimal example

```rust
// Call this constructor in `view`, then map events to Message variants.
```

## How it works

Constructors return typed widget values. You configure behavior via builder methods and emit `Message` values for `update` to handle.

## Common patterns

```rust
// Keep constructor calls close to their message mapping.
// Prefer small helper functions for repeated widget setups.
```

## Gotchas / tips

- Re-check argument order in the verified signature on this page.
- Keep side effects out of `view`; trigger them from `update` with Task.
- Use the related family page when deciding between module/element APIs.

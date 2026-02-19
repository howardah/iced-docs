---
title: Element - Button
description: Struct reference for iced::widget::Button.
version: latest
last_updated: 2026-02-19
order: 502
---

# Element - Button

Authoritative source: ref/doc/iced/widget/struct.Button.html.

## Rustdoc summary

A generic widget that produces a message when pressed.

## Verified type declaration

```rust
pub struct Button<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/game_of_life/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/sandpiles/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::button;

#[derive(Clone)]
enum Message {
    ButtonPressed,
}

fn view(state: &State) -> Element<'_, Message> {
    button("Press me!").on_press(Message::ButtonPressed).into()
}
```

```rust
use iced::widget::button;

#[derive(Clone)]
enum Message {
    ButtonPressed,
}

fn view(state: &State) -> Element<'_, Message> {
    button("I am disabled!").into()
}
```

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need the concrete widget struct type in signatures.
- You are debugging type errors involving generic bounds.
- You want lower-level control than constructor-only docs provide.

## Minimal example

```rust
// Constructors usually produce this element type.
// Name the type explicitly only when type-level APIs need it.
```

## How it works

Element structs are the underlying widget types used by constructors. Most app code can stay constructor-first, then use element docs for advanced typing/customization.

## Common patterns

```rust
// Use constructors in normal UI code,
// and reserve explicit element types for reusable abstractions.
```

## Gotchas / tips

- You usually do not need to construct element structs directly.
- Read trait bounds carefully when adding custom renderer/theme types.
- If a method is missing, check the related module page.

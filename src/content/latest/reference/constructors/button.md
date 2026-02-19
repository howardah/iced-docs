---
title: Constructor - Button
description: Function reference for iced::widget::button.
version: latest
last_updated: 2026-02-19
order: 304
---

# Constructor - Button

Authoritative source: `ref/doc/iced/widget/fn.button.html`.

## Rustdoc summary

Creates a new
Button
with the provided content.

## Verified signature

```rust
pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/websocket/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/layout/src/main.rs
- ref/examples/exit/src/main.rs

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

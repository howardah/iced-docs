---
title: Constructor - Toggler
description: Function reference for iced::widget::toggler.
version: latest
last_updated: 2026-02-19
order: 341
---

# Constructor - Toggler

Authoritative source: `ref/doc/iced/widget/fn.toggler.html`.

## Rustdoc summary

Creates a new
Toggler
.

## Verified signature

```rust
pub fn toggler<'a, Message, Theme, Renderer>(
    is_checked: bool,
) -> Toggler<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/custom_quad/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/qr_code/src/main.rs
- ref/examples/styling/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::toggler;

struct State {
   is_checked: bool,
}

enum Message {
    TogglerToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    toggler(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::TogglerToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::TogglerToggled(is_checked) => {
            state.is_checked = is_checked;
        }
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

---
title: Element - Toggler
description: Struct reference for iced::widget::Toggler.
version: latest
last_updated: 2026-02-19
order: 528
---

# Element - Toggler

Authoritative source: ref/doc/iced/widget/struct.Toggler.html.

## Rustdoc summary

A toggler widget.

## Verified type declaration

```rust
pub struct Toggler<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/tour/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs
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

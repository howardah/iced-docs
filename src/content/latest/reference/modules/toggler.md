---
title: Module - Toggler
description: Module-level reference for iced::widget::toggler.
version: latest
last_updated: 2026-02-19
order: 126
---

# Module - Toggler

Authoritative source: `ref/doc/iced/widget/toggler/index.html`.

## Rustdoc description

Togglers let users make binary choices by toggling a switch.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::toggler.

## Example References

- ref/examples/custom_quad/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/qr_code/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/tour/src/main.rs

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

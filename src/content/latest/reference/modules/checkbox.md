---
title: Module - Checkbox
description: Module-level reference for iced::widget::checkbox.
version: latest
last_updated: 2026-02-19
order: 102
---

# Module - Checkbox

Authoritative source: `ref/doc/iced/widget/checkbox/index.html`.

## Rustdoc description

Checkboxes can be used to let users make binary choices.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::checkbox.

## Example References

- ref/examples/checkbox/src/main.rs
- ref/examples/custom_shader/src/main.rs
- ref/examples/events/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/gradient/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::checkbox;

struct State {
   is_checked: bool,
}

enum Message {
    CheckboxToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    checkbox(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::CheckboxToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::CheckboxToggled(is_checked) => {
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

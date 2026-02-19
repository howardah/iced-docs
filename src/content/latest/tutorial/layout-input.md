---
title: Tutorial 2 - Layout and Input
description: Compose rows/columns and collect user input with text_input.
version: latest
last_updated: 2026-02-19
order: 2
---

# Tutorial 2 - Layout and Input

This step adds composition (`column!`, `row!`) and controlled input (`text_input`) so your app can gather user data.

## Use this when...

- You are moving from buttons-only UI to forms.
- You need layout composition and event wiring together.
- You want a clean pattern for controlled text fields.

## Minimal example

```rust
use iced::widget::{column, row, text, text_input};

let content = column![
    text("Todo"),
    row![text("Left"), text("Right")],
    text_input("What needs to be done?", &state.input)
        .on_input(Message::InputChanged)
        .on_submit(Message::CreateTask),
];
```

## How it works

`text_input` is controlled: your state owns the current string value. On each edit, Iced sends a message. You store the new value in `update`, and `view` rerenders with the updated string.

## Common patterns

```rust
fn update(state: &mut App, message: Message) {
    match message {
        Message::InputChanged(value) => state.input = value,
        Message::CreateTask => {
            if !state.input.is_empty() {
                state.tasks.push(state.input.clone());
                state.input.clear();
            }
        }
    }
}
```

## Gotchas / tips

- Always feed `text_input` from state; avoid temporary local strings.
- Validate and trim input in `update`, not in `view`.
- Use `row!`/`column!` helpers early; they keep layout readable.

## Next

- [Tutorial 3 - Async Tasks](/latest/tutorial/async-tasks)
- [Widgets Overview](/latest/reference/widgets-overview)

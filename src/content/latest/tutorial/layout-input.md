---
title: Tutorial 2 - Layout and Input
description: Compose rows/columns and collect user input with text_input.
version: latest
last_updated: 2026-02-19
order: 2
---

# Tutorial 2 - Layout and Input

This tutorial uses layout and input patterns shown in `ref/examples/tour` and `ref/examples/todos`.

## Layout with row and column

```rust
use iced::widget::{column, row, text};

let content = column![
    text("Todo"),
    row![text("Left"), text("Right")]
];
```

## Text input

Verified constructor from `ref/doc/iced/widget/fn.text_input.html`:

```rust
pub fn text_input<'a, Message, Theme, Renderer>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme, Renderer>
```

Typical usage in examples:

```rust
text_input("What needs to be done?", input_value)
    .on_input(Message::InputChanged)
    .on_submit(Message::CreateTask)
```

## What you learned

- `row!` and `column!` build container layout quickly
- `text_input` maps text edits into messages

## Next

- [Tutorial 3 - Async Tasks](/latest/tutorial/async-tasks)
- [Reference: Widgets Overview](/latest/reference/widgets-overview)

---
title: Runtime Function - exit
description: Detailed guidance for iced::exit.
version: latest
last_updated: 2026-02-19
order: 23
---

# Runtime Function - `iced::exit`

Authoritative source: `ref/doc/iced/fn.exit.html`.

## Verified signature

```rust
pub fn exit<T>() -> Task<T>
```

## Use this when...

- A user action should close the app.
- Business logic decides runtime should stop.
- You want shutdown to stay inside normal task/message flow.

## Minimal example

```rust
fn update(_state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::QuitConfirmed => iced::exit(),
        _ => iced::Task::none(),
    }
}
```

## How it works

`exit` returns a task instead of terminating immediately. This keeps shutdown behavior aligned with Iced's side-effect model.

## Common patterns

```rust
match message {
    Message::ExitClicked => {
        state.show_confirm = true;
        iced::Task::none()
    }
    Message::ExitConfirmed => iced::exit(),
    Message::ExitCanceled => {
        state.show_confirm = false;
        iced::Task::none()
    }
}
```

## Gotchas / tips

- Call `exit` from `update`, not from `view`.
- Consider confirmation UI before shutdown for destructive workflows.
- Keep cleanup-triggering messages explicit so behavior is testable.

## Example references

- `ref/examples/exit/src/main.rs`
- `ref/examples/changelog/src/main.rs`
- `ref/examples/multi_window/src/main.rs`

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Runtime Function - daemon](/latest/reference/runtime-fn-daemon)

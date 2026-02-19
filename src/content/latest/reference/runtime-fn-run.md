---
title: Runtime Function - run
description: Detailed guidance for iced::run.
version: latest
last_updated: 2026-02-19
order: 25
---

# Runtime Function - `iced::run`

Authoritative source: `ref/doc/iced/fn.run.html`.

## Verified signature

```rust
pub fn run<State, Message, Theme, Renderer>(
    update: impl UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
) -> Result
where
    State: Default + 'static,
    Message: Send + MaybeDebug + MaybeClone + 'static,
    Theme: Base + 'static,
    Renderer: Renderer + 'static,
```

## Use this when...

- You want the shortest path to a working app.
- `State: Default` is acceptable.
- You do not need heavy runtime builder configuration yet.

## Minimal example

```rust
pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
```

## How it works

`run` wires your `update` and `view` functions into Iced's event loop. User input creates messages, `update` mutates state, and `view` rebuilds widgets from that state.

## Common patterns

```rust
use iced::widget::{button, column, text};

#[derive(Default)]
struct App { value: u64 }

#[derive(Debug, Clone)]
enum Message { Increment }

impl App {
    fn update(&mut self, message: Message) {
        match message { Message::Increment => self.value += 1 }
    }

    fn view(&self) -> iced::widget::Column<'_, Message> {
        column![text(self.value), button("+").on_press(Message::Increment)]
    }
}
```

## Gotchas / tips

- `run` requires default-initializable state; use `application` when bootstrapping is custom.
- Keep `view` side-effect free; perform async work via tasks in `update`.
- If startup config grows, migrate to `iced::application(...)` rather than overloading state defaults.

## Example references

- `ref/examples/counter/src/main.rs`
- `ref/examples/progress_bar/src/main.rs`
- `ref/examples/exit/src/main.rs`

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Runtime Function - application](/latest/reference/runtime-fn-application)

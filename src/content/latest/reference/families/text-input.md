---
title: Family - Text Input
description: Unified reference for the Text Input widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 745
---

# Family - Text Input

This page unifies related iced::widget APIs for the **Text Input** family.

## API surfaces

- Module: [iced::widget::text_input](/latest/reference/modules/text_input)
- Constructor: [iced::widget::text_input](/latest/reference/constructors/text_input)
- Element: [iced::widget::TextInput](/latest/reference/elements/text-input)

## Surface summaries

### Module

Text inputs display fields that can be filled with text.

### Constructor

Creates a new
TextInput
.

### Element

A field that can be filled with text.

## Verified constructor signature

```rust
pub fn text_input<'a, Message, Theme, Renderer>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct TextInput<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/qr_code/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/toast/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/todos/src/main.rs
- ref/examples/tour/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::text_input;

struct State {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String)
}

fn view(state: &State) -> Element<'_, Message> {
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}
```

### Element example

```rust
use iced::widget::text_input;

struct State {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String)
}

fn view(state: &State) -> Element<'_, Message> {
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

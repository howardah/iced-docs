---
title: Family - Span
description: Unified reference for the Span widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 740
---

# Family - Span

This page unifies related iced::widget APIs for the **Span** family.

## API surfaces

- Constructor: [iced::widget::span](/latest/reference/constructors/span)

## Surface summaries

### Constructor

Creates a new
Span
of text with the provided content.

## Verified constructor signature

```rust
pub fn span<'a, Link, Font>(text: impl IntoFragment<'a>) -> Span<'a, Link, Font>
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/todos/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::font;
use iced::widget::{rich_text, span};
use iced::{color, never, Font};

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    rich_text![
        span("I am red!").color(color!(0xff0000)),
        " ",
        span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
    ]
    .on_link_click(never)
    .size(20)
    .into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

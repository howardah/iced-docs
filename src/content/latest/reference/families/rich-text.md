---
title: Family - Rich Text
description: Unified reference for the Rich Text widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 730
---

# Family - Rich Text

This page unifies related iced::widget APIs for the **Rich Text** family.

## API surfaces

- Constructor: [iced::widget::rich_text](/latest/reference/constructors/rich_text)

## Surface summaries

### Constructor

Creates a new
Rich
text widget with the provided spans.

## Verified constructor signature

```rust
pub fn rich_text<'a, Link, Message, Theme, Renderer>(
    spans: impl AsRef<[Span<'a, Link, <Renderer as Renderer>::Font>]> + 'a,
) -> Rich<'a, Link, Message, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog + 'a,
    Renderer: Renderer,
    <Renderer as Renderer>::Font: 'a,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/geometry/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::font;
use iced::widget::{rich_text, span};
use iced::{color, never, Font};

#[derive(Debug, Clone)]
enum Message {
    LinkClicked(&'static str),
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    rich_text([
        span("I am red!").color(color!(0xff0000)),
        span(" "),
        span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
    ])
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

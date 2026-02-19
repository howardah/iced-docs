---
title: Family - Text
description: Unified reference for the Text widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 743
---

# Family - Text

This page unifies related iced::widget APIs for the **Text** family.

## API surfaces

- Module: [iced::widget::text](/latest/reference/modules/text)
- Constructor: [iced::widget::text](/latest/reference/constructors/text)

## Surface summaries

### Module

Draw and interact with text.

### Constructor

Creates a new
Text
widget with the provided content.

## Verified constructor signature

```rust
pub fn text<'a, Theme, Renderer>(
    text: impl IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/custom_shader/src/scene/camera.rs
- ref/examples/custom_shader/src/scene/pipeline.rs
- ref/examples/editor/src/main.rs
- ref/examples/stopwatch/src/main.rs
- ref/examples/tooltip/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::text;
use iced::color;

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    text("Hello, this is iced!")
        .size(20)
        .color(color!(0x0000ff))
        .into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

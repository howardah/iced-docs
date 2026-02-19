---
title: Constructor - Text
description: Function reference for iced::widget::text.
version: latest
last_updated: 2026-02-19
order: 337
---

# Constructor - Text

Authoritative source: `ref/doc/iced/widget/fn.text.html`.

## Rustdoc summary

Creates a new
Text
widget with the provided content.

## Verified signature

```rust
pub fn text<'a, Theme, Renderer>(
    text: impl IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/counter/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/websocket/src/main.rs
- ref/examples/integration/src/controls.rs
- ref/examples/loading_spinners/src/main.rs

## Inline Examples (from rustdoc)

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

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

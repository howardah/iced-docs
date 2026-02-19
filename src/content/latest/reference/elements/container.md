---
title: Element - Container
description: Struct reference for iced::widget::Container.
version: latest
last_updated: 2026-02-19
order: 506
---

# Element - Container

Authoritative source: ref/doc/iced/widget/struct.Container.html.

## Rustdoc summary

A widget that aligns its contents inside of its boundaries.

## Verified type declaration

```rust
pub struct Container<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/gradient/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/arc/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/checkbox/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::container;

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    container("This text is centered inside a rounded box!")
        .padding(10)
        .center(800)
        .style(container::rounded_box)
        .into()
}
```

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

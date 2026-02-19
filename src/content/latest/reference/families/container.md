---
title: Family - Container
description: Unified reference for the Container widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 712
---

# Family - Container

This page unifies related iced::widget APIs for the **Container** family.

## API surfaces

- Module: [iced::widget::container](/latest/reference/modules/container)
- Constructor: [iced::widget::container](/latest/reference/constructors/container)
- Element: [iced::widget::Container](/latest/reference/elements/container)

## Surface summaries

### Module

Containers let you align a widget inside their boundaries.

### Constructor

Creates a new
Container
with the provided content.

### Element

A widget that aligns its contents inside of its boundaries.

## Verified constructor signature

```rust
pub fn container<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Container<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/markdown/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

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

### Element example

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

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

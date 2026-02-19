---
title: Widgets Overview
description: Coverage map for widget families, modules, constructors, and element structs.
version: latest
last_updated: 2026-02-19
order: 3
---

# Widgets Overview

Iced widget docs in this site are organized by API surface so you can learn quickly and then drill down.

## Use this when...

- You are not sure whether to start from a module, constructor, element, or family page.
- You want a path from "build UI quickly" to "customize deeply".
- You want to see related APIs for the same widget in one place.

## Minimal example

Most widgets are easiest to start from constructors:

```rust
use iced::widget::{button, column, text_input};

let ui = column![
    text_input("Name", "").on_input(Message::NameChanged),
    button("Save").on_press(Message::Save),
];
```

## How it works

- Constructor pages focus on practical widget creation.
- Element pages cover the underlying struct types and advanced control.
- Module pages cover family-specific traits/types.
- Family pages connect all of the above for one widget domain.

For beginners, start with **Families** and **Constructors**.

## Common patterns

As apps grow, split widget-building into helper functions that return `Element<Message>`:

```rust
fn action_row<'a>() -> iced::Element<'a, Message> {
    iced::widget::row![
        iced::widget::button("Cancel").on_press(Message::Cancel),
        iced::widget::button("Save").on_press(Message::Save),
    ]
    .spacing(8)
    .into()
}
```

## Gotchas / tips

- Constructor and element names are often related but serve different documentation goals.
- If an option is unclear, check the module page for style/state traits.
- Use family pages to avoid missing related APIs.

## Catalog pages

- [Families](/latest/reference/families)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
- [Modules](/latest/reference/modules)
- [Structs](/latest/reference/structs)
- [Enums](/latest/reference/enums)

## Related

- [Core Concepts](/latest/reference/core-concepts)
- [Runtime API](/latest/reference/runtime-api)

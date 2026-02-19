---
title: Tutorial 4 - Theming and Components
description: Scale UI by extracting reusable view functions and using theme hooks.
version: latest
last_updated: 2026-02-19
order: 4
---

# Tutorial 4 - Theming and Components

As your app grows, extracting reusable widget builders and centralizing theme choices keeps `view` code readable.

## Use this when...

- Your `view` function is getting too large.
- You need consistent button/field styling.
- You want runtime-level theme decisions.

## Minimal example

```rust
use iced::widget::{button, text};

fn padded_button<'a>(label: &'a str) -> iced::widget::Button<'a, Message> {
    button(text(label)).padding([12, 24])
}
```

## How it works

Component extraction in Iced is usually just Rust functions returning widgets or `Element<Message>`. Theme selection often lives at app boot (`application(...).theme(...)`) and style details live near widget construction.

## Common patterns

```rust
pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .run()
}
```

## Gotchas / tips

- Keep extracted UI helpers message-typed so event flow remains obvious.
- Prefer small reusable builders over deeply nested giant `view` trees.
- Set theme in one place to avoid style drift.

## Continue

- [Guide: Distribution](/latest/guide/distribution)
- [Core Concepts](/latest/reference/core-concepts)

---
title: Tutorial 4 - Theming and Components
description: Scale UI by extracting reusable view functions and using theme hooks.
version: latest
last_updated: 2026-02-19
order: 4
---

# Tutorial 4 - Theming and Components

When apps grow, keep business state in one place and extract reusable view helpers.

## Reusable component function

```rust
fn padded_button<'a>(label: &'a str) -> Button<'a, Message> {
    button(text(label).align_x(Center)).padding(12)
}
```

This pattern appears in `ref/examples/tour/src/main.rs`.

## Theme and style hooks

Use builder methods like `.theme(...)` on application setup when you need custom theme selection.

## What you learned

- Decompose large `view` trees into helper functions
- Keep messages explicit between parent and child UI pieces
- Centralize theming decisions at app configuration boundaries

## Continue

- [Guide: Distribution](/latest/guide/distribution)
- [Reference: Core Concepts](/latest/reference/core-concepts)

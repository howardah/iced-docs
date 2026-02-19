---
title: Constructor - Tooltip
description: Function reference for iced::widget::tooltip.
version: latest
last_updated: 2026-02-19
order: 342
---

# Constructor - Tooltip

Authoritative source: `ref/doc/iced/widget/fn.tooltip.html`.

## Rustdoc summary

Creates a new
Tooltip
for the provided content with the given

Element
and
tooltip::Position
.

## Verified signature

```rust
pub fn tooltip<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    tooltip: impl Into<Element<'a, Message, Theme, Renderer>>,
    position: Position,
) -> Tooltip<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/tooltip/src/main.rs
- ref/examples/table/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{container, tooltip};

enum Message {
    // ...
}

fn view(_state: &State) -> Element<'_, Message> {
    tooltip(
        "Hover me to display the tooltip!",
        container("This is the tooltip contents!")
            .padding(10)
            .style(container::rounded_box),
        tooltip::Position::Bottom,
    ).into()
}
```

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want the canonical entrypoint for creating this widget/helper.
- You need concrete constructor arguments and builder chaining.
- You are wiring UI interactions into typed messages.

## Minimal example

```rust
// Call this constructor in `view`, then map events to Message variants.
```

## How it works

Constructors return typed widget values. You configure behavior via builder methods and emit `Message` values for `update` to handle.

## Common patterns

```rust
// Keep constructor calls close to their message mapping.
// Prefer small helper functions for repeated widget setups.
```

## Gotchas / tips

- Re-check argument order in the verified signature on this page.
- Keep side effects out of `view`; trigger them from `update` with Task.
- Use the related family page when deciding between module/element APIs.

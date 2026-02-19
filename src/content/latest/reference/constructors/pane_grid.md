---
title: Constructor - Pane Grid
description: Function reference for iced::widget::pane_grid.
version: latest
last_updated: 2026-02-19
order: 319
---

# Constructor - Pane Grid

Authoritative source: `ref/doc/iced/widget/fn.pane_grid.html`.

## Rustdoc summary

Creates a
PaneGrid
with the given
pane_grid::State
and view function.

## Verified signature

```rust
pub fn pane_grid<'a, T, Message, Theme, Renderer>(
    state: &'a State<T>,
    view: impl Fn(Pane, &'a T, bool) -> Content<'a, Message, Theme, Renderer>,
) -> PaneGrid<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/pane_grid/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{pane_grid, text};

struct State {
    panes: pane_grid::State<Pane>,
}

enum Pane {
    SomePane,
    AnotherKindOfPane,
}

enum Message {
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
}

fn view(state: &State) -> Element<'_, Message> {
    pane_grid(&state.panes, |pane, state, is_maximized| {
        pane_grid::Content::new(match state {
            Pane::SomePane => text("This is some pane"),
            Pane::AnotherKindOfPane => text("This is another kind of pane"),
        })
    })
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .into()
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

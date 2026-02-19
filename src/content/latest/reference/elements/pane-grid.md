---
title: Element - Pane Grid
description: Struct reference for iced::widget::PaneGrid.
version: latest
last_updated: 2026-02-19
order: 511
---

# Element - Pane Grid

Authoritative source: ref/doc/iced/widget/struct.PaneGrid.html.

## Rustdoc summary

A collection of panes distributed using either vertical or horizontal splits
to completely fill the space available.

## Verified type declaration

```rust
pub struct PaneGrid<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

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

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need the concrete widget struct type in signatures.
- You are debugging type errors involving generic bounds.
- You want lower-level control than constructor-only docs provide.

## Minimal example

```rust
// Constructors usually produce this element type.
// Name the type explicitly only when type-level APIs need it.
```

## How it works

Element structs are the underlying widget types used by constructors. Most app code can stay constructor-first, then use element docs for advanced typing/customization.

## Common patterns

```rust
// Use constructors in normal UI code,
// and reserve explicit element types for reusable abstractions.
```

## Gotchas / tips

- You usually do not need to construct element structs directly.
- Read trait bounds carefully when adding custom renderer/theme types.
- If a method is missing, check the related module page.

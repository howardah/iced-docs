---
title: Module - Pane Grid
description: Module-level reference for iced::widget::pane_grid.
version: latest
last_updated: 2026-02-19
order: 110
---

# Module - Pane Grid

Authoritative source: `ref/doc/iced/widget/pane_grid/index.html`.

## Rustdoc description

Pane grids let your users split regions of your application and organize layout dynamically.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::pane_grid.

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

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

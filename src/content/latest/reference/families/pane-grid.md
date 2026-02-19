---
title: Family - Pane Grid
description: Unified reference for the Pane Grid widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 724
---

# Family - Pane Grid

This page unifies related iced::widget APIs for the **Pane Grid** family.

## API surfaces

- Module: [iced::widget::pane_grid](/latest/reference/modules/pane_grid)
- Constructor: [iced::widget::pane_grid](/latest/reference/constructors/pane_grid)
- Element: [iced::widget::PaneGrid](/latest/reference/elements/pane-grid)

## Surface summaries

### Module

Pane grids let your users split regions of your application and organize layout dynamically.

### Constructor

Creates a
PaneGrid
with the given
pane_grid::State
and view function.

### Element

A collection of panes distributed using either vertical or horizontal splits
to completely fill the space available.

## Verified constructor signature

```rust
pub fn pane_grid<'a, T, Message, Theme, Renderer>(
    state: &'a State<T>,
    view: impl Fn(Pane, &'a T, bool) -> Content<'a, Message, Theme, Renderer>,
) -> PaneGrid<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct PaneGrid<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/pane_grid/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

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

### Element example

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

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want one page that links module, constructor, and element surfaces.
- You are deciding which API surface to start from.
- You need a practical map for this widget domain.

## Minimal example

```rust
// Typical flow:
// 1) Start with constructor usage.
// 2) Move to module docs for style/state details.
// 3) Use element docs for type-level control.
```

## How it works

Family pages connect related docs so you do not miss capabilities that are split across constructor/module/element pages.

## Common patterns

```rust
// Build with constructor APIs first,
// then refine behavior/styles through related module and element docs.
```

## Gotchas / tips

- Family routes normalize naming; module/function/struct names may differ slightly.
- Prefer this page as your entrypoint when learning unfamiliar widgets.
- Follow example references here before inventing integration patterns.

---
title: Family - Grid
description: Unified reference for the Grid widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 714
---

# Family - Grid

This page unifies related iced::widget APIs for the **Grid** family.

## API surfaces

- Module: [iced::widget::grid](/latest/reference/modules/grid)
- Constructor: [iced::widget::grid](/latest/reference/constructors/grid)
- Element: [iced::widget::Grid](/latest/reference/elements/grid)

## Surface summaries

### Module

Distribute content on a grid.

### Constructor

Creates a new
Grid
from an iterator.

### Element

Creates a keyed
Column
with the given children.

## Verified constructor signature

```rust
pub fn grid<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Grid<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Grid<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/game_of_life/src/main.rs
- ref/examples/sandpiles/src/main.rs
- ref/examples/gallery/src/main.rs

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

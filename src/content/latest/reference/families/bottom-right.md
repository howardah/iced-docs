---
title: Family - Bottom Right
description: Unified reference for the Bottom Right widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 704
---

# Family - Bottom Right

This page unifies related iced::widget APIs for the **Bottom Right** family.

## API surfaces

- Constructor: [iced::widget::bottom_right](/latest/reference/constructors/bottom_right)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
and aligns its contents inside to the bottom right corner.

## Verified constructor signature

```rust
pub fn bottom_right<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/loading_spinners/src/circular.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let bottom_right = container("Bottom!").align_right(Fill).align_bottom(Fill);
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

---
title: Family - Responsive
description: Unified reference for the Responsive widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 729
---

# Family - Responsive

This page unifies related iced::widget APIs for the **Responsive** family.

## API surfaces

- Constructor: [iced::widget::responsive](/latest/reference/constructors/responsive)
- Element: [iced::widget::Responsive](/latest/reference/elements/responsive)

## Surface summaries

### Constructor

Creates a new
Responsive
widget with a closure that produces its
contents.

### Element

A widget that is aware of its dimensions.

## Verified constructor signature

```rust
pub fn responsive<'a, Message, Theme, Renderer>(
    f: impl Fn(Size) -> Element<'a, Message, Theme, Renderer> + 'a,
) -> Responsive<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Responsive<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/pane_grid/src/main.rs
- ref/examples/layout/src/main.rs

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

---
title: Family - Float
description: Unified reference for the Float widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 713
---

# Family - Float

This page unifies related iced::widget APIs for the **Float** family.

## API surfaces

- Module: [iced::widget::float](/latest/reference/modules/float)
- Constructor: [iced::widget::float](/latest/reference/constructors/float)
- Element: [iced::widget::Float](/latest/reference/elements/float)

## Surface summaries

### Module

Make elements float!

### Constructor

Creates a new
Float
widget with the given content.

### Element

A widget that can make its contents float over other widgets.

## Verified constructor signature

```rust
pub fn float<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Float<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Float<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/custom_shader/src/scene/pipeline.rs
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

---
title: Family - Hover
description: Unified reference for the Hover widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 715
---

# Family - Hover

This page unifies related iced::widget APIs for the **Hover** family.

## API surfaces

- Constructor: [iced::widget::hover](/latest/reference/constructors/hover)

## Surface summaries

### Constructor

Displays a widget on top of another one, only when the base widget is hovered.

## Verified constructor signature

```rust
pub fn hover<'a, Message, Theme, Renderer>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    top: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/the_matrix/src/main.rs
- ref/examples/solar_system/src/main.rs
- ref/examples/stopwatch/src/main.rs

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

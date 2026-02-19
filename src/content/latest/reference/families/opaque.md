---
title: Family - Opaque
description: Unified reference for the Opaque widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 721
---

# Family - Opaque

This page unifies related iced::widget APIs for the **Opaque** family.

## API surfaces

- Constructor: [iced::widget::opaque](/latest/reference/constructors/opaque)

## Surface summaries

### Constructor

Wraps the given widget and captures any mouse button presses inside the bounds of
the widget—effectively making it opaque.

## Verified constructor signature

```rust
pub fn opaque<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/checkbox/src/main.rs
- ref/examples/pane_grid/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/delineate/src/main.rs

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

---
title: Family - Center X
description: Unified reference for the Center X widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 707
---

# Family - Center X

This page unifies related iced::widget APIs for the **Center X** family.

## API surfaces

- Constructor: [iced::widget::center_x](/latest/reference/constructors/center_x)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
horizontally and centers its contents inside.

## Verified constructor signature

```rust
pub fn center_x<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/arc/src/main.rs
- ref/examples/combo_box/src/main.rs
- ref/examples/pick_list/src/main.rs
- ref/examples/todos/src/main.rs
- ref/examples/editor/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let center_x = container("Horizontal Center!").center_x(Fill);
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

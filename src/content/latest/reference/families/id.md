---
title: Family - Id
description: Unified reference for the Id widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 717
---

# Family - Id

This page unifies related iced::widget APIs for the **Id** family.

## API surfaces

- Element: [iced::widget::Id](/latest/reference/elements/id)

## Surface summaries

### Element

The identifier of a generic widget.

## Verified element declaration

```rust
pub struct Id(/* private fields */);
```
## Example References

- ref/examples/editor/src/main.rs
- ref/examples/todos/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/scrollable/src/main.rs

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

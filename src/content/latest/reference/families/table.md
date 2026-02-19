---
title: Family - Table
description: Unified reference for the Table widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 742
---

# Family - Table

This page unifies related iced::widget APIs for the **Table** family.

## API surfaces

- Module: [iced::widget::table](/latest/reference/modules/table)
- Constructor: [iced::widget::table](/latest/reference/constructors/table)

## Surface summaries

### Module

Display tables.

### Constructor

Creates a new
Table
with the given columns and rows.

## Verified constructor signature

```rust
pub fn table<'a, 'b, T, Message, Theme, Renderer>(
    columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
    rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer>
where
    T: Clone,
    Theme: Catalog,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/events/src/main.rs

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

---
title: Family - Operation
description: Unified reference for the Operation widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 722
---

# Family - Operation

This page unifies related iced::widget APIs for the **Operation** family.

## API surfaces

- Module: [iced::widget::operation](/latest/reference/modules/operation)

## Surface summaries

### Module

Change internal widget state.
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/game_of_life/src/preset.rs
- ref/examples/qr_code/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/the_matrix/src/main.rs

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

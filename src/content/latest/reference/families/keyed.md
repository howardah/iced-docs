---
title: Family - Keyed
description: Unified reference for the Keyed widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 718
---

# Family - Keyed

This page unifies related iced::widget APIs for the **Keyed** family.

## API surfaces

- Module: [iced::widget::keyed](/latest/reference/modules/keyed)

## Surface summaries

### Module

Keyed widgets can provide hints to ensure continuity.
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/gallery/src/civitai.rs
- ref/examples/editor/src/main.rs
- ref/examples/geometry/src/main.rs

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

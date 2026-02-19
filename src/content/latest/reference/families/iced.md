---
title: Family - Iced
description: Unified reference for the Iced widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 716
---

# Family - Iced

This page unifies related iced::widget APIs for the **Iced** family.

## API surfaces

- Constructor: [iced::widget::iced](/latest/reference/constructors/iced)

## Surface summaries

### Constructor

Creates an
Element
that displays the iced logo with the given text_size.

## Verified constructor signature

```rust
pub fn iced<'a, Message, Theme, Renderer>(
    text_size: impl Into<Pixels>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: Renderer + Renderer<Font = Font> + 'a,
    Theme: Catalog + Catalog + 'a,
    <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
    <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/tooltip/src/main.rs
- ref/examples/pick_list/src/main.rs
- ref/examples/combo_box/src/main.rs
- ref/examples/custom_shader/src/scene/camera.rs

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

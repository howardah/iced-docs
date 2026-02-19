---
title: Family - Themer
description: Unified reference for the Themer widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 747
---

# Family - Themer

This page unifies related iced::widget APIs for the **Themer** family.

## API surfaces

- Constructor: [iced::widget::themer](/latest/reference/constructors/themer)
- Element: [iced::widget::Themer](/latest/reference/elements/themer)

## Surface summaries

### Constructor

A widget that applies any Theme to its contents.

### Element

A widget that applies any Theme to its contents.

## Verified constructor signature

```rust
pub fn themer<'a, Message, Theme, Renderer>(
    theme: Option<Theme>,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Themer<'a, Message, Theme, Renderer>
where
    Theme: Base,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Themer<'a, Message, Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- TODO(api-verify): add canonical example mapping for this item.

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

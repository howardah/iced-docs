---
title: Family - Shader
description: Unified reference for the Shader widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 737
---

# Family - Shader

This page unifies related iced::widget APIs for the **Shader** family.

## API surfaces

- Module: [iced::widget::shader](/latest/reference/modules/shader)
- Constructor: [iced::widget::shader](/latest/reference/constructors/shader)
- Element: [iced::widget::Shader](/latest/reference/elements/shader)

## Surface summaries

### Module

A custom shader widget for wgpu applications.

### Constructor

Creates a new
Shader
.

### Element

A widget which can render custom shaders with Iced’s wgpu backend.

## Verified constructor signature

```rust
pub fn shader<Message, P>(program: P) -> Shader<Message, P>
where
    P: Program<Message>,
```

## Verified element declaration

```rust
pub struct Shader<Message, P>
where
    P: Program<Message>,{ /* private fields */ }
```
## Example References

- ref/examples/custom_shader/src/main.rs
- ref/examples/custom_shader/src/scene.rs

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

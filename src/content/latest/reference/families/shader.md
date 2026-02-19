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

---
title: Element - Shader
description: Struct reference for iced::widget::Shader.
version: latest
last_updated: 2026-02-19
order: 521
---

# Element - Shader

Authoritative source: ref/doc/iced/widget/struct.Shader.html.

## Rustdoc summary

A widget which can render custom shaders with Iced’s wgpu backend.

## Verified type declaration

```rust
pub struct Shader<Message, P>
where
    P: Program<Message>,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/custom_shader/src/main.rs


## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need the concrete widget struct type in signatures.
- You are debugging type errors involving generic bounds.
- You want lower-level control than constructor-only docs provide.

## Minimal example

```rust
// Constructors usually produce this element type.
// Name the type explicitly only when type-level APIs need it.
```

## How it works

Element structs are the underlying widget types used by constructors. Most app code can stay constructor-first, then use element docs for advanced typing/customization.

## Common patterns

```rust
// Use constructors in normal UI code,
// and reserve explicit element types for reusable abstractions.
```

## Gotchas / tips

- You usually do not need to construct element structs directly.
- Read trait bounds carefully when adding custom renderer/theme types.
- If a method is missing, check the related module page.

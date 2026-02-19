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
    P: Program<Message>, {/* private fields */}
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

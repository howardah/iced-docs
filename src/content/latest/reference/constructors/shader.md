---
title: Constructor - Shader
description: Function reference for iced::widget::shader.
version: latest
last_updated: 2026-02-19
order: 331
---

# Constructor - Shader

Authoritative source: ref/doc/iced/widget/fn.shader.html.

## Rustdoc summary

Creates a new
Shader
.

## Verified signature

```rust
pub fn shader<Message, P>(program: P) -> Shader<Message, P>where
P: Program<Message>,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/custom_shader/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

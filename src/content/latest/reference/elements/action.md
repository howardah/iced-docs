---
title: Element - Action
description: Struct reference for iced::widget::Action.
version: latest
last_updated: 2026-02-19
order: 501
---

# Element - Action

Authoritative source: ref/doc/iced/widget/struct.Action.html.

## Rustdoc summary

A runtime action that can be performed by some widgets.

## Verified type declaration

```rust
pub struct Action<Message> { /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/multitouch/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs


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

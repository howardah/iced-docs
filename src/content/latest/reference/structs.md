---
title: Structs
description: Index of crate-level iced structs exposed in rustdoc.
version: latest
last_updated: 2026-02-19
order: 95
---

# Structs

Crate-level structs include geometry/style primitives (`Point`, `Size`, `Color`, `Border`) and runtime building blocks (`Task`, `Subscription`, `Settings`).

## Use this when...

- You need explicit Iced types in app state or helper APIs.
- You are configuring runtime/layout/styling with strongly typed values.
- You want to understand trait bounds and ownership of core runtime types.

## Minimal example

```rust
let padding = iced::Padding::from(12);
let color = iced::Color::from_rgb8(0x33, 0x66, 0x99);
```

## How it works

These types are used across many modules, so understanding them improves all widget/runtime work. Runtime structs (`Task`, `Subscription`, `Settings`) are especially important for app architecture.

## Common patterns

```rust
fn refresh() -> iced::Task<Message> {
    iced::Task::perform(load(), Message::Loaded)
}
```

## Gotchas / tips

- Use Iced structs directly instead of loosely typed primitives when available.
- Keep runtime structs near app bootstrap/update logic, not deeply buried in UI helpers.
- Geometry/style structs are cheap clarity wins for maintainability.

## Struct Index

- [Animation](/latest/reference/structs/animation)
- [Border](/latest/reference/structs/border)
- [Color](/latest/reference/structs/color)
- [Degrees](/latest/reference/structs/degrees)
- [Font](/latest/reference/structs/font)
- [Padding](/latest/reference/structs/padding)
- [Pixels](/latest/reference/structs/pixels)
- [Point](/latest/reference/structs/point)
- [Preset](/latest/reference/structs/preset)
- [Radians](/latest/reference/structs/radians)
- [Rectangle](/latest/reference/structs/rectangle)
- [Settings](/latest/reference/structs/settings)
- [Shadow](/latest/reference/structs/shadow)
- [Size](/latest/reference/structs/size)
- [Subscription](/latest/reference/structs/subscription)
- [Task](/latest/reference/structs/task)
- [Transformation](/latest/reference/structs/transformation)
- [Vector](/latest/reference/structs/vector)

## Related

- [Enums](/latest/reference/enums)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)

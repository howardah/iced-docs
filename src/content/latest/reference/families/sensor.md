---
title: Family - Sensor
description: Unified reference for the Sensor widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 736
---

# Family - Sensor

This page unifies related iced::widget APIs for the **Sensor** family.

## API surfaces

- Module: [iced::widget::sensor](/latest/reference/modules/sensor)
- Constructor: [iced::widget::sensor](/latest/reference/constructors/sensor)
- Element: [iced::widget::Sensor](/latest/reference/elements/sensor)

## Surface summaries

### Module

Generate messages when content pops in and out of view.

### Constructor

Creates a new
Sensor
widget.

### Element

A widget that can generate messages when its content pops in and out of view.

## Verified constructor signature

```rust
pub fn sensor<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Sensor<'a, (), Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Sensor<'a, Key, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/markdown/src/main.rs
- ref/examples/gallery/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

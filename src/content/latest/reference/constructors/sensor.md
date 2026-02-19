---
title: Constructor - Sensor
description: Function reference for iced::widget::sensor.
version: latest
last_updated: 2026-02-19
order: 330
---

# Constructor - Sensor

Authoritative source: `ref/doc/iced/widget/fn.sensor.html`.

## Rustdoc summary

Creates a new
Sensor
widget.

## Verified signature

```rust
pub fn sensor<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Sensor<'a, (), Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/gallery/src/main.rs
- ref/examples/markdown/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

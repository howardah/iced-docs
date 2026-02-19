---
title: Constructor - Span
description: Function reference for iced::widget::span.
version: latest
last_updated: 2026-02-19
order: 334
---

# Constructor - Span

Authoritative source: `ref/doc/iced/widget/fn.span.html`.

## Rustdoc summary

Creates a new
Span
of text with the provided content.

## Verified signature

```rust
pub fn span<'a, Link, Font>(text: impl IntoFragment<'a>) -> Span<'a, Link, Font>
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/tour/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

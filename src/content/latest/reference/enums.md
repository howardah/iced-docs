---
title: Enums
description: Index of crate-level iced enums exposed in rustdoc.
version: latest
last_updated: 2026-02-19
order: 94
---

# Enums

Crate-level enums in Iced represent key configuration strategies (layout sizing, event types, background/gradient options, theme-level choices, and more).

## Use this when...

- You need to pick between fixed configuration variants.
- You want type-safe pattern matching in app logic.
- You are learning cross-cutting runtime/layout primitives.

## Minimal example

```rust
use iced::Length;

let width = Length::Fill;
let height = Length::Fixed(32.0);
```

## How it works

Most enums are passed into widget builders or runtime helpers. Because they are strongly typed, compiler errors usually guide you to valid choices quickly.

## Common patterns

```rust
match state.compact_mode {
    true => iced::Length::Shrink,
    false => iced::Length::Fill,
}
```

## Gotchas / tips

- Similar variants can have different behavior depending on widget context.
- Prefer enum variants over ad-hoc booleans for readability.
- Check per-enum pages for concrete examples from official apps.

## Enum Index

- [Alignment](/latest/reference/enums/alignment)
- [Background](/latest/reference/enums/background)
- [ContentFit](/latest/reference/enums/content-fit)
- [Error](/latest/reference/enums/error)
- [Event](/latest/reference/enums/event)
- [Gradient](/latest/reference/enums/gradient)
- [Length](/latest/reference/enums/length)
- [Never](/latest/reference/enums/never)
- [Rotation](/latest/reference/enums/rotation)
- [Theme](/latest/reference/enums/theme)

## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)

---
title: Enum - Gradient
description: Enum reference for iced::Gradient.
version: latest
last_updated: 2026-02-19
order: 76
---

# Enum - Gradient

Authoritative source: `ref/doc/iced/enum.Gradient.html`.

## Rustdoc summary

A fill which transitions colors progressively along a direction, either linearly, radially (TBD), or conically (TBD).

## Verified declaration

```rust
pub enum Gradient {
    Linear(Linear),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/gradient/src/main.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

---
title: Enum - Rotation
description: Enum reference for iced::Rotation.
version: latest
last_updated: 2026-02-19
order: 79
---

# Enum - Rotation

Authoritative source: `ref/doc/iced/enum.Rotation.html`.

## Rustdoc summary

The strategy used to rotate the content.

## Verified declaration

```rust
pub enum Rotation {
    Floating(Radians),
    Solid(Radians),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/ferris/src/main.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

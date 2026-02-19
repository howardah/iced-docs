---
title: Enum - Background
description: Enum reference for iced::Background.
version: latest
last_updated: 2026-02-19
order: 72
---

# Enum - Background

Authoritative source: `ref/doc/iced/enum.Background.html`.

## Rustdoc summary

The background of some element.

## Verified declaration

```rust
pub enum Background {
    Color(Color),
    Gradient(Gradient),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/loading_spinners/src/circular.rs
- ref/examples/loading_spinners/src/linear.rs
- ref/examples/integration/src/controls.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

---
title: Enum - ContentFit
description: Enum reference for iced::ContentFit.
version: latest
last_updated: 2026-02-19
order: 73
---

# Enum - ContentFit

Authoritative source: `ref/doc/iced/enum.ContentFit.html`.

## Rustdoc summary

The strategy used to fit the contents of a widget to its bounding box.

## Verified declaration

```rust
pub enum ContentFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/ferris/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/screenshot/src/main.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

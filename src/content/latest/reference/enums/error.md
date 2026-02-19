---
title: Enum - Error
description: Enum reference for iced::Error.
version: latest
last_updated: 2026-02-19
order: 74
---

# Enum - Error

Authoritative source: `ref/doc/iced/enum.Error.html`.

## Rustdoc summary

An error that occurred while running an application.

## Verified declaration

```rust
pub enum Error {
    ExecutorCreationFailed(Error),
    WindowCreationFailed(Box<dyn Error + Send + Sync>),
    GraphicsCreationFailed(Error),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/download_progress/src/download.rs
- ref/examples/changelog/src/changelog.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

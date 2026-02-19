---
title: Enum - Never
description: Enum reference for iced::Never.
version: latest
last_updated: 2026-02-19
order: 78
---

# Enum - Never

Authoritative source: `ref/doc/iced/enum.Never.html`.

## Rustdoc summary

The error type for errors that can never happen.

## Verified declaration

```rust
pub enum Never {}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/geometry/src/main.rs
- ref/examples/websocket/src/echo.rs

## Inline Examples (from rustdoc)

```rust
impl<T, U> TryFrom<U> for T where U: Into<T> {
    type Error = Infallible;

    fn try_from(value: U) -> Result<Self, Infallible> {
        Ok(U::into(value))  // Never returns `Err`
    }
}
```

```rust
pub type Infallible = !;
```

## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

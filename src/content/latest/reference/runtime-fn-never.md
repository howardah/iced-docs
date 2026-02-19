---
title: Runtime Function - never
description: Use iced::never to bridge unreachable Infallible code paths.
version: latest
last_updated: 2026-02-19
order: 25
---

# Runtime Function - iced::never

Authoritative source: `ref/doc/iced/fn.never.html`.

## Verified signature

```rust
pub fn never<T>(never: Infallible) -> T
```

## What it is for

`never` converts an `Infallible` value into any target type. Since `Infallible` has no values, this function is for unreachable branches in typed control flow.

## When to use

- Advanced generic or message-mapping code where an impossible case must still satisfy a type.

## Why this exists

It provides a standard helper for expressing unreachable logic without unsafe code.

## Caution

This is not a normal app-entry function and is rarely needed in day-to-day UI code.

## Related

- [Runtime API](/latest/reference/runtime-api)

---
title: Runtime Function - never
description: Detailed guidance for iced::never.
version: latest
last_updated: 2026-02-19
order: 24
---

# Runtime Function - iced::never

Authoritative source: ref/doc/iced/fn.never.html.

## Verified signature

```rust
pub fn never<T>(never: Infallible) -> T
```
## When to use it

Use it only for advanced unreachable `Infallible`-based branches in typed/generic code.

## Why to use it

It allows impossible branches to satisfy type requirements safely.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)

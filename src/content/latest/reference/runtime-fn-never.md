---
title: Runtime Function - never
description: Detailed guidance for iced::never.
version: latest
last_updated: 2026-02-19
order: 24
---

# Runtime Function - `iced::never`

Authoritative source: `ref/doc/iced/fn.never.html`.

## Verified signature

```rust
pub fn never<T>(never: Infallible) -> T
```

## Use this when...

- You are working with advanced generic code involving `Infallible`.
- You must satisfy type constraints for an unreachable branch.
- You need a compile-time proof that a code path cannot happen.

## Minimal example

```rust
use std::convert::Infallible;

fn impossible(value: Infallible) -> i32 {
    iced::never(value)
}
```

## How it works

`never` converts an `Infallible` value into any type because such a value can never exist at runtime. This is mainly useful in generic/adaptor code.

## Common patterns

```rust
fn map_result<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        Err(never) => iced::never(never),
    }
}
```

## Gotchas / tips

- This is an advanced utility; most app code never needs it.
- Prefer clearer APIs over introducing `Infallible` unless generics demand it.
- If unsure, avoid using this function and simplify the type flow first.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)

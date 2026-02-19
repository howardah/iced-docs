---
title: Struct - Animation
description: Struct reference for iced::Animation.
version: latest
last_updated: 2026-02-19
order: 81
---

# Struct - Animation

Authoritative source: `ref/doc/iced/struct.Animation.html`.

## Rustdoc summary

The animation of some particular state.

## Verified declaration

```rust
pub struct Animation<T>
where
    T: Clone + Copy + PartialEq + FloatRepresentable,{ /* private fields */ }
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/loading_spinners/src/circular.rs
- ref/examples/gallery/src/main.rs
- ref/examples/markdown/src/main.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)

## Use this when...

- You need this concrete Iced type in state/configuration/helpers.
- You want stronger typing than primitive values provide.
- You are working with runtime primitives like Task/Subscription/Settings.

## Minimal example

```rust
// Construct and pass this struct where the corresponding API expects it.
```

## How it works

Crate-level structs define shared runtime, geometry, styling, and configuration data. Using them directly keeps app code aligned with rustdoc contracts.

## Common patterns

```rust
// Centralize commonly reused struct values in helper constructors.
```

## Gotchas / tips

- Prefer explicit Iced structs over loosely typed primitives where possible.
- Check trait bounds when using these types in generic code.
- For runtime structs, keep lifecycle ownership clear.

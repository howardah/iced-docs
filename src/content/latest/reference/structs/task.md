---
title: Struct - Task
description: Struct reference for iced::Task.
version: latest
last_updated: 2026-02-19
order: 96
---

# Struct - Task

Authoritative source: `ref/doc/iced/struct.Task.html`.

## Rustdoc summary

A set of concurrent actions to be performed by the iced runtime.

## Verified declaration

```rust
pub struct Task<T> { /* private fields */ }
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/websocket/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/system_information/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/modal/src/main.rs


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

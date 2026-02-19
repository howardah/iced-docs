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

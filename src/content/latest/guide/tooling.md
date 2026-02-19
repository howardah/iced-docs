---
title: Tooling
description: Local development loops for native and web iced applications.
version: latest
last_updated: 2026-02-19
order: 4
---

# Tooling

Use `cargo run` for native loops and Trunk for web-capable examples.

## Native loop

```sh
cargo run --package tour
```

## Web loop

```sh
cd ref/examples/tour
trunk serve
```

## Debugging and tracing

Some examples initialize tracing on native targets:

```rust
#[cfg(not(target_arch = "wasm32"))]
tracing_subscriber::fmt::init();
```

## Related

- [Bundling](/latest/guide/bundling)
- [Tutorial 3: Async Tasks](/latest/tutorial/async-tasks)

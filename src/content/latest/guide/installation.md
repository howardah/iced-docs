---
title: Installation
description: Install Rust and add iced for native or web workflows.
version: latest
last_updated: 2026-02-19
order: 2
---

# Installation

Install Rust using rustup, then add `iced` to your crate dependencies.

## Cargo dependency

```toml
[dependencies]
iced = "0.14"
```

## Confirm your environment

- `cargo --version`
- `rustc --version`

## Run a known-good example

The official examples are under `ref/examples`.

```sh
cargo run --package counter
```

## Notes

For web targets, many examples use Trunk.

```sh
cd ref/examples/todos
trunk serve
```

## Related

- [Project setup](/latest/guide/project-setup)
- [Tutorial 1](/latest/tutorial/basic-window-button)

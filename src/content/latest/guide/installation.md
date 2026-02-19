---
title: Installation
description: Install Rust and add iced for native or web workflows.
version: latest
last_updated: 2026-02-19
order: 2
---

# Installation

Install Rust, create a project, add `iced`, and verify your toolchain by running a known-good example.

## Use this when...

- You are setting up Iced for the first time.
- You need a clean baseline before debugging app code.
- You want both native and web-ready workflows available.

## Minimal example

```toml
[dependencies]
iced = "0.14"
```

## How it works

`iced` compiles for native and wasm targets, but your exact flow depends on where you run the app. For local desktop development, `cargo run` is enough. For web examples, many projects use Trunk.

## Common patterns

```sh
rustup update
cargo --version
rustc --version
cargo run --package counter
```

## Gotchas / tips

- Match crate and docs versions; API differences across versions are significant.
- Confirm `rustup` target/tooling before debugging runtime issues.
- Validate setup by running official examples from `ref/examples`.

## Related

- [Project setup](/latest/guide/project-setup)
- [Tutorial 1 - Basic Window and Button](/latest/tutorial/basic-window-button)

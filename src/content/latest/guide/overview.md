---
title: Guide Overview
description: Start here to navigate installation, setup, tooling, bundling, and distribution for iced.
version: latest
last_updated: 2026-02-19
order: 1
---

# Guide Overview

This guide is a practical path from a fresh Rust environment to a packaged Iced app.

## Use this when...

- You are new to Iced and want a step-by-step adoption path.
- You need to set up a reliable local workflow.
- You are preparing to ship desktop or web builds.

## Minimal example

```rust
pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
```

## How it works

Start with installation and project setup so message flow is clear early. Then add tooling and runtime configuration. Finally, package and validate target-specific output.

## Common patterns

A realistic development loop:

```sh
cargo run --package counter
cargo check
cargo test
```

## Gotchas / tips

- Verify behavior against official examples before introducing custom architecture.
- Keep app state/message structure simple early; complexity compounds quickly.
- Separate UI rendering from side effects from day one.

## Continue

- [Installation](/latest/guide/installation)
- [Project setup](/latest/guide/project-setup)
- [Tooling](/latest/guide/tooling)
- [Bundling](/latest/guide/bundling)
- [Distribution](/latest/guide/distribution)

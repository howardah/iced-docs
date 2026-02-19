---
title: Tooling
description: Local development loops for native and web iced applications.
version: latest
last_updated: 2026-02-19
order: 4
---

# Tooling

Use fast feedback loops (`cargo check`, targeted example runs, tests) while building Iced apps.

## Use this when...

- You want reliable edit-build-run loops.
- You are debugging widget behavior against official examples.
- You need a repeatable pre-commit validation routine.

## Minimal example

```sh
cargo check
cargo run --package tour
```

## How it works

Most Iced iteration is native `cargo run` plus `cargo check`. For wasm examples, Trunk is a practical local server workflow.

## Common patterns

```sh
cd ref/examples/tour
trunk serve
```

## Gotchas / tips

- `cargo check` catches most typing/import issues faster than full runs.
- Confirm example package names before assuming your environment is broken.
- Keep logs/tracing setup gated for native vs wasm targets.

## Related

- [Bundling](/latest/guide/bundling)
- [Tutorial 3 - Async Tasks](/latest/tutorial/async-tasks)

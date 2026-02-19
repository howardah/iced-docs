---
title: Runtime API
description: Top-level iced runtime entrypoints and how to choose between them.
version: latest
last_updated: 2026-02-19
order: 2
---

# Runtime API

Iced exposes five top-level runtime functions in rustdoc:

- `application`
- `daemon`
- `exit`
- `never`
- `run`

Authoritative source list:

- `ref/doc/iced/index.html`
- `ref/doc/iced/fn.application.html`
- `ref/doc/iced/fn.daemon.html`
- `ref/doc/iced/fn.exit.html`
- `ref/doc/iced/fn.never.html`
- `ref/doc/iced/fn.run.html`

## How to choose

- Start with [`run`](/latest/reference/runtime-fn-run) for simple apps.
- Use [`application`](/latest/reference/runtime-fn-application) when you need startup/configuration hooks.
- Use [`daemon`](/latest/reference/runtime-fn-daemon) for background-driven multi-window or utility workflows.
- Use [`exit`](/latest/reference/runtime-fn-exit) to terminate runtime from update logic.
- Use [`never`](/latest/reference/runtime-fn-never) only for unreachable code paths involving `Infallible`.

## Per-function pages

- [Runtime Function - run](/latest/reference/runtime-fn-run)
- [Runtime Function - application](/latest/reference/runtime-fn-application)
- [Runtime Function - daemon](/latest/reference/runtime-fn-daemon)
- [Runtime Function - exit](/latest/reference/runtime-fn-exit)
- [Runtime Function - never](/latest/reference/runtime-fn-never)

## Related

- [Core Concepts](/latest/reference/core-concepts)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)

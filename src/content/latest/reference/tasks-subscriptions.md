---
title: Tasks and Subscriptions
description: Use Task for async actions and Subscription for passive event streams.
version: latest
last_updated: 2026-02-19
order: 4
---

# Tasks and Subscriptions

Rustdoc defines:

- `Task<T>` as concurrent actions performed by the iced runtime
- `Subscription<T>` as requests to listen to external events

## Task

`Task` is used heavily in async flows like the `todos` example.

Practical methods to know (verify exact bounds in rustdoc):

- `Task::none`
- `Task::perform`
- `Task::batch`
- `Task::map`
- `Task::chain`
- `Task::abortable`

## Subscription

Common composition methods:

- `Subscription::none`
- `Subscription::run`
- `Subscription::run_with`
- `Subscription::batch`
- `Subscription::map`

## Usage guidance

- Prefer `Task` for finite async work (HTTP request, file IO)
- Prefer `Subscription` for ongoing streams (input/events/time)
- Keep all outputs mapped into your main `Message` enum

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Tutorial 3 - Async Tasks](/latest/tutorial/async-tasks)

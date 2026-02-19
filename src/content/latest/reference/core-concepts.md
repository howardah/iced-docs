---
title: Core Concepts
description: Mental model for state, messages, update, view, tasks, and subscriptions in iced.
version: latest
last_updated: 2026-02-19
order: 1
---

# Core Concepts

Iced apps are message-driven.

## State and Message

- State stores UI and domain data
- Message represents user or runtime events

## Update

`update` handles messages and mutates state.

Depending on app setup, update may also return a `Task<Message>` for async work.

## View

`view` reads state and returns widget trees.

## Task and Subscription

- `Task<T>`: one-off or chained async actions
- `Subscription<T>`: ongoing external event requests

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)

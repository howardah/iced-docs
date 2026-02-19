---
title: Widgets Overview
description: Common widgets and constructors referenced across official examples.
version: latest
last_updated: 2026-02-19
order: 3
---

# Widgets Overview

The module `iced::widget` exposes widgets, layout macros, and helper constructors.

## High-usage widgets in examples

- `button`
- `text`
- `text_input`
- `checkbox`
- `slider`
- `scrollable`
- `container`
- `row` and `column`

## Verified constructor examples

From rustdoc:

```rust
pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>
```

```rust
pub fn text_input<'a, Message, Theme, Renderer>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme, Renderer>
```

## When to use

- Use `row!` and `column!` for primary layout
- Use `container` for spacing/alignment wrappers
- Use `scrollable` when content can overflow

## Related

- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
- [Tutorial 2 - Layout and Input](/latest/tutorial/layout-input)

---
title: Widget Constructor - stack
description: Function reference for iced::widget::stack.
version: latest
last_updated: 2026-02-19
order: 335
---

# Widget Constructor - iced::widget::stack

Authoritative source: ref/doc/iced/widget/fn.stack.html.

## Rustdoc summary

Creates a new
Stack
with the given children.

## Verified signature

```rust
pub fn stack<'a, Message, Theme, Renderer>(
children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Stack<'a, Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

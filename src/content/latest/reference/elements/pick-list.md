---
title: Element - Pick List
description: Struct reference for iced::widget::PickList.
version: latest
last_updated: 2026-02-19
order: 512
---

# Element - Pick List

Authoritative source: ref/doc/iced/widget/struct.PickList.html.

## Rustdoc summary

A widget for selecting a single value from a list of options.

## Verified type declaration

```rust
pub struct PickList<'a, T, L, V, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>where
T: ToString + PartialEq + Clone,
L: Borrow<[T]> + 'a,
V: Borrow<T> + 'a,
Theme: Catalog,
Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

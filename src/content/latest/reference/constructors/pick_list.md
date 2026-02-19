---
title: Constructor - Pick List
description: Function reference for iced::widget::pick_list.
version: latest
last_updated: 2026-02-19
order: 320
---

# Constructor - Pick List

Authoritative source: `ref/doc/iced/widget/fn.pick_list.html`.

## Rustdoc summary

Creates a new
PickList
.

## Verified signature

```rust
pub fn pick_list<'a, T, L, V, Message, Theme, Renderer>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: Catalog + Catalog,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/changelog/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

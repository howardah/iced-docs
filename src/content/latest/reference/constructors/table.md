---
title: Constructor - Table
description: Function reference for iced::widget::table.
version: latest
last_updated: 2026-02-19
order: 336
---

# Constructor - Table

Authoritative source: `ref/doc/iced/widget/fn.table.html`.

## Rustdoc summary

Creates a new
Table
with the given columns and rows.

## Verified signature

```rust
pub fn table<'a, 'b, T, Message, Theme, Renderer>(
    columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
    rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer>
where
    T: Clone,
    Theme: Catalog,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/table/src/main.rs


## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want the canonical entrypoint for creating this widget/helper.
- You need concrete constructor arguments and builder chaining.
- You are wiring UI interactions into typed messages.

## Minimal example

```rust
// Call this constructor in `view`, then map events to Message variants.
```

## How it works

Constructors return typed widget values. You configure behavior via builder methods and emit `Message` values for `update` to handle.

## Common patterns

```rust
// Keep constructor calls close to their message mapping.
// Prefer small helper functions for repeated widget setups.
```

## Gotchas / tips

- Re-check argument order in the verified signature on this page.
- Keep side effects out of `view`; trigger them from `update` with Task.
- Use the related family page when deciding between module/element APIs.

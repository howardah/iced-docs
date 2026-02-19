---
title: Constructor - Themer
description: Function reference for iced::widget::themer.
version: latest
last_updated: 2026-02-19
order: 340
---

# Constructor - Themer

Authoritative source: `ref/doc/iced/widget/fn.themer.html`.

## Rustdoc summary

A widget that applies any Theme to its contents.

## Verified signature

```rust
pub fn themer<'a, Message, Theme, Renderer>(
    theme: Option<Theme>,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Themer<'a, Message, Theme, Renderer>
where
    Theme: Base,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- TODO(api-verify): add canonical example mapping for this item.


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

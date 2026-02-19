---
title: Enum - Gradient
description: Enum reference for iced::Gradient.
version: latest
last_updated: 2026-02-19
order: 76
---

# Enum - Gradient

Authoritative source: `ref/doc/iced/enum.Gradient.html`.

## Rustdoc summary

A fill which transitions colors progressively along a direction, either linearly, radially (TBD), or conically (TBD).

## Verified declaration

```rust
pub enum Gradient {
    Linear(Linear),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/gradient/src/main.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

## Use this when...

- You need a typed set of variants for layout/style/runtime behavior.
- You want pattern matching instead of ad-hoc booleans.
- You are configuring widget behavior with explicit choices.

## Minimal example

```rust
// Choose an enum variant and pass it into widget/runtime configuration.
```

## How it works

Enums model constrained strategy choices in Iced APIs. They keep configuration readable and make invalid combinations easier to catch at compile time.

## Common patterns

```rust
// Use match expressions for app-driven variant selection.
```

## Gotchas / tips

- Similar variant names may have different effects across APIs.
- Keep variant selection close to widget config for clarity.
- Confirm semantics in rustdoc for edge cases.

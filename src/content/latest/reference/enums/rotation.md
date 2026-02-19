---
title: Enum - Rotation
description: Enum reference for iced::Rotation.
version: latest
last_updated: 2026-02-19
order: 79
---

# Enum - Rotation

Authoritative source: `ref/doc/iced/enum.Rotation.html`.

## Rustdoc summary

The strategy used to rotate the content.

## Verified declaration

```rust
pub enum Rotation {
    Floating(Radians),
    Solid(Radians),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/ferris/src/main.rs


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

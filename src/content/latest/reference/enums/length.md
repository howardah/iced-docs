---
title: Enum - Length
description: Enum reference for iced::Length.
version: latest
last_updated: 2026-02-19
order: 77
---

# Enum - Length

Authoritative source: `ref/doc/iced/enum.Length.html`.

## Rustdoc summary

The strategy used to fill space in a specific dimension.

## Verified declaration

```rust
pub enum Length {
    Fill,
    FillPortion(u16),
    Shrink,
    Fixed(f32),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/toast/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/loading_spinners/src/circular.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/loading_spinners/src/linear.rs
- ref/examples/layout/src/main.rs


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

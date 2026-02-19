---
title: Enum - ContentFit
description: Enum reference for iced::ContentFit.
version: latest
last_updated: 2026-02-19
order: 73
---

# Enum - ContentFit

Authoritative source: `ref/doc/iced/enum.ContentFit.html`.

## Rustdoc summary

The strategy used to fit the contents of a widget to its bounding box.

## Verified declaration

```rust
pub enum ContentFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/ferris/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/screenshot/src/main.rs


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

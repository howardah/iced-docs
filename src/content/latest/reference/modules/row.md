---
title: Module - Row
description: Module-level reference for iced::widget::row.
version: latest
last_updated: 2026-02-19
order: 114
---

# Module - Row

Authoritative source: `ref/doc/iced/widget/row/index.html`.

## Rustdoc description

Distribute content horizontally.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::row.

## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/layout/src/main.rs
- ref/examples/styling/src/main.rs


## Related

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need module-level APIs beyond the basic constructor call.
- You want family-specific style/state traits and helper types.
- You are building reusable widget abstractions.

## Minimal example

```rust
// Start with the constructor from this module family in `view`.
// Then move to module APIs for deeper customization.
```

## How it works

Module docs explain the namespace that groups constructors, types, and related traits. In everyday app code, this helps you discover advanced options after basic usage works.

## Common patterns

```rust
// Message flow pattern:
// widget interaction -> Message -> update -> state change -> rerender
```

## Gotchas / tips

- Check this page together with its family page for complete context.
- Verify trait bounds and associated types in rustdoc when custom styling fails.
- Keep module imports explicit while learning.

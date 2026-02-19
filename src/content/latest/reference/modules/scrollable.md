---
title: Module - Scrollable
description: Module-level reference for iced::widget::scrollable.
version: latest
last_updated: 2026-02-19
order: 116
---

# Module - Scrollable

Authoritative source: `ref/doc/iced/widget/scrollable/index.html`.

## Rustdoc description

Scrollables let users navigate an endless amount of content with a scrollbar.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::scrollable.

## Example References

- ref/examples/scrollable/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/combo_box/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/geometry/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{column, scrollable, space};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    scrollable(column![
        "Scroll me!",
        space().height(3000),
        "You did it!",
    ]).into()
}
```

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

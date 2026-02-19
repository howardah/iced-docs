---
title: Module - Tooltip
description: Module-level reference for iced::widget::tooltip.
version: latest
last_updated: 2026-02-19
order: 127
---

# Module - Tooltip

Authoritative source: `ref/doc/iced/widget/tooltip/index.html`.

## Rustdoc description

Tooltips display a hint of information over some element when hovered.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::tooltip.

## Example References

- ref/examples/tooltip/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/table/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{container, tooltip};

enum Message {
    // ...
}

fn view(_state: &State) -> Element<'_, Message> {
    tooltip(
        "Hover me to display the tooltip!",
        container("This is the tooltip contents!")
            .padding(10)
            .style(container::rounded_box),
        tooltip::Position::Bottom,
    ).into()
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

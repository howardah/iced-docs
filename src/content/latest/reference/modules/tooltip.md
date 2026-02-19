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

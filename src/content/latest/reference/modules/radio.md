---
title: Module - Radio
description: Module-level reference for iced::widget::radio.
version: latest
last_updated: 2026-02-19
order: 113
---

# Module - Radio

Authoritative source: `ref/doc/iced/widget/radio/index.html`.

## Rustdoc description

Radio buttons let users choose a single option from a bunch of options.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::radio.

## Example References

- ref/examples/scrollable/src/main.rs
- ref/examples/tour/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{column, radio};

struct State {
   selection: Option<Choice>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(Choice),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    A,
    B,
    C,
    All,
}

fn view(state: &State) -> Element<'_, Message> {
    let a = radio(
        "A",
        Choice::A,
        state.selection,
        Message::RadioSelected,
    );

    let b = radio(
        "B",
        Choice::B,
        state.selection,
        Message::RadioSelected,
    );

    let c = radio(
        "C",
        Choice::C,
        state.selection,
        Message::RadioSelected,
    );

    let all = radio(
        "All of the above",
        Choice::All,
        state.selection,
        Message::RadioSelected
    );

    column![a, b, c, all].into()
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

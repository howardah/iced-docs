---
title: Module - Rule
description: Module-level reference for iced::widget::rule.
version: latest
last_updated: 2026-02-19
order: 115
---

# Module - Rule

Authoritative source: `ref/doc/iced/widget/rule/index.html`.

## Rustdoc description

Rules divide space horizontally or vertically.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::rule.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Inline Examples (from rustdoc)

```rust
use iced::widget::rule;

#[derive(Clone)]
enum Message {
    // ...,
}

fn view(state: &State) -> Element<'_, Message> {
    rule::horizontal(2).into()
}
```

## Related

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

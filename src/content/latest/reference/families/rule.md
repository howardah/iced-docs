---
title: Family - Rule
description: Unified reference for the Rule widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 734
---

# Family - Rule

This page unifies related iced::widget APIs for the **Rule** family.

## API surfaces

- Module: [iced::widget::rule](/latest/reference/modules/rule)
- Element: [iced::widget::Rule](/latest/reference/elements/rule)

## Surface summaries

### Module

Rules divide space horizontally or vertically.

### Element

Display a horizontal or vertical rule for dividing content.

## Verified element declaration

```rust
pub struct Rule<'a, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Inline Examples (from rustdoc)

### Element example

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

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

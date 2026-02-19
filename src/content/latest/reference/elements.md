---
title: Elements
description: Index of all iced::widget element structs exposed in rustdoc.
version: latest
last_updated: 2026-02-19
order: 92
---

# Elements

Element pages document the underlying widget struct types. Use them when constructor-level documentation is not enough.

## Use this when...

- You need explicit widget types in signatures or helper APIs.
- You are reading trait bounds and advanced builder behavior.
- You need exact struct-level docs from rustdoc.

## Minimal example

```rust
use iced::widget::button;

let action: iced::widget::Button<'_, Message> = button("Run").on_press(Message::Run);
```

## How it works

Constructors usually produce these element structs. You can often stay at constructor level, then move here for deeper control or type-level integration.

## Common patterns

```rust
fn toolbar<'a>() -> iced::Element<'a, Message> {
    iced::widget::row![
        iced::widget::button("Open").on_press(Message::Open),
        iced::widget::button("Save").on_press(Message::Save),
    ]
    .spacing(8)
    .into()
}
```

## Gotchas / tips

- You rarely need to name element struct types directly in early code.
- When type errors are confusing, check these pages for full generic bounds.
- Prefer constructors in app code and element types in reusable abstractions.

## Element Index

- [Action](/latest/reference/elements/action)
- [Button](/latest/reference/elements/button)
- [Checkbox](/latest/reference/elements/checkbox)
- [Column](/latest/reference/elements/column)
- [Combo Box](/latest/reference/elements/combo-box)
- [Container](/latest/reference/elements/container)
- [Float](/latest/reference/elements/float)
- [Grid](/latest/reference/elements/grid)
- [Id](/latest/reference/elements/id)
- [Mouse Area](/latest/reference/elements/mouse-area)
- [Pane Grid](/latest/reference/elements/pane-grid)
- [Pick List](/latest/reference/elements/pick-list)
- [Pin](/latest/reference/elements/pin)
- [Progress Bar](/latest/reference/elements/progress-bar)
- [Radio](/latest/reference/elements/radio)
- [Responsive](/latest/reference/elements/responsive)
- [Row](/latest/reference/elements/row)
- [Rule](/latest/reference/elements/rule)
- [Scrollable](/latest/reference/elements/scrollable)
- [Sensor](/latest/reference/elements/sensor)
- [Shader](/latest/reference/elements/shader)
- [Slider](/latest/reference/elements/slider)
- [Space](/latest/reference/elements/space)
- [Stack](/latest/reference/elements/stack)
- [Text Editor](/latest/reference/elements/text-editor)
- [Text Input](/latest/reference/elements/text-input)
- [Themer](/latest/reference/elements/themer)
- [Toggler](/latest/reference/elements/toggler)
- [Tooltip](/latest/reference/elements/tooltip)
- [Vertical Slider](/latest/reference/elements/vertical-slider)

## Related

- [Constructors](/latest/reference/constructors)
- [Families](/latest/reference/families)

---
title: Families
description: Unified widget-family pages that group module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 93
---

# Families

Family pages are the best starting point for most widgets. Each family groups the module, constructor, and element pages for one concept.

## Use this when...

- You are learning a widget and want all relevant API surfaces together.
- You need to move from basic usage to deeper customization.
- You want direct links between constructor/module/element docs.

## Minimal example

```rust
// Typical workflow:
// 1) Start with constructor docs.
// 2) Move to module docs for style/state details.
// 3) Use element docs for type-level specifics.
```

## How it works

Most families correspond to one widget domain (for example, `button`, `text_input`, `scrollable`). Layout helpers and utility families are included too.

## Common patterns

```rust
let content = iced::widget::column![
    iced::widget::text_input("Search", &state.query).on_input(Message::QueryChanged),
    iced::widget::button("Run").on_press(Message::Run),
    iced::widget::scrollable(state.results_view()),
];
```

## Gotchas / tips

- Family names are normalized; paths may use hyphens or underscores depending on surface.
- Use family pages first when you are unsure where an API belongs.
- If one surface lacks detail, follow linked sibling pages in the same family.

## Family Index

- [Action](/latest/reference/families/action)
- [Bottom](/latest/reference/families/bottom)
- [Bottom Center](/latest/reference/families/bottom-center)
- [Bottom Right](/latest/reference/families/bottom-right)
- [Button](/latest/reference/families/button)
- [Center](/latest/reference/families/center)
- [Center X](/latest/reference/families/center-x)
- [Center Y](/latest/reference/families/center-y)
- [Checkbox](/latest/reference/families/checkbox)
- [Column](/latest/reference/families/column)
- [Combo Box](/latest/reference/families/combo-box)
- [Container](/latest/reference/families/container)
- [Float](/latest/reference/families/float)
- [Grid](/latest/reference/families/grid)
- [Hover](/latest/reference/families/hover)
- [Iced](/latest/reference/families/iced)
- [Id](/latest/reference/families/id)
- [Keyed](/latest/reference/families/keyed)
- [Keyed Column](/latest/reference/families/keyed-column)
- [Mouse Area](/latest/reference/families/mouse-area)
- [Opaque](/latest/reference/families/opaque)
- [Operation](/latest/reference/families/operation)
- [Overlay](/latest/reference/families/overlay)
- [Pane Grid](/latest/reference/families/pane-grid)
- [Pick List](/latest/reference/families/pick-list)
- [Pin](/latest/reference/families/pin)
- [Progress Bar](/latest/reference/families/progress-bar)
- [Radio](/latest/reference/families/radio)
- [Responsive](/latest/reference/families/responsive)
- [Rich Text](/latest/reference/families/rich-text)
- [Right](/latest/reference/families/right)
- [Right Center](/latest/reference/families/right-center)
- [Row](/latest/reference/families/row)
- [Rule](/latest/reference/families/rule)
- [Scrollable](/latest/reference/families/scrollable)
- [Sensor](/latest/reference/families/sensor)
- [Shader](/latest/reference/families/shader)
- [Slider](/latest/reference/families/slider)
- [Space](/latest/reference/families/space)
- [Span](/latest/reference/families/span)
- [Stack](/latest/reference/families/stack)
- [Table](/latest/reference/families/table)
- [Text](/latest/reference/families/text)
- [Text Editor](/latest/reference/families/text-editor)
- [Text Input](/latest/reference/families/text-input)
- [Theme](/latest/reference/families/theme)
- [Themer](/latest/reference/families/themer)
- [Toggler](/latest/reference/families/toggler)
- [Tooltip](/latest/reference/families/tooltip)
- [Value](/latest/reference/families/value)
- [Vertical Slider](/latest/reference/families/vertical-slider)

## Related

- [Widgets Overview](/latest/reference/widgets-overview)
- [Core Concepts](/latest/reference/core-concepts)

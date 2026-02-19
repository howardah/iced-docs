---
title: Modules
description: Index of all iced::widget modules exposed in rustdoc.
version: latest
last_updated: 2026-02-19
order: 90
---

# Modules

Module pages document widget namespaces like `iced::widget::button`, including family-specific style and helper APIs.

## Use this when...

- You need deeper APIs than the free constructor call.
- You want to discover module-level style/state types.
- You are building reusable abstractions around one widget family.

## Minimal example

```rust
use iced::widget::button;

let save = button("Save").on_press(Message::Save);
```

## How it works

In practice, you often start from constructors and then come back to module pages for advanced options (theme/style catalogs, helper types, and integration points).

## Common patterns

```rust
use iced::widget::{button, container, text_input};

let form = container(
    iced::widget::column![
        text_input("Name", &state.name).on_input(Message::NameChanged),
        button("Submit").on_press(Message::Submit),
    ]
);
```

## Gotchas / tips

- Constructor and module names can differ slightly (`combo_box`, `text_input`, etc.).
- If a constructor page seems sparse, check its module page for richer context.
- Keep imports explicit while learning; it clarifies which API surface you are using.

## Module Index

- [Button](/latest/reference/modules/button)
- [Checkbox](/latest/reference/modules/checkbox)
- [Combo Box](/latest/reference/modules/combo_box)
- [Container](/latest/reference/modules/container)
- [Float](/latest/reference/modules/float)
- [Grid](/latest/reference/modules/grid)
- [Keyed](/latest/reference/modules/keyed)
- [Operation](/latest/reference/modules/operation)
- [Overlay](/latest/reference/modules/overlay)
- [Pane Grid](/latest/reference/modules/pane_grid)
- [Pick List](/latest/reference/modules/pick_list)
- [Progress Bar](/latest/reference/modules/progress_bar)
- [Radio](/latest/reference/modules/radio)
- [Row](/latest/reference/modules/row)
- [Rule](/latest/reference/modules/rule)
- [Scrollable](/latest/reference/modules/scrollable)
- [Sensor](/latest/reference/modules/sensor)
- [Shader](/latest/reference/modules/shader)
- [Slider](/latest/reference/modules/slider)
- [Space](/latest/reference/modules/space)
- [Table](/latest/reference/modules/table)
- [Text](/latest/reference/modules/text)
- [Text Editor](/latest/reference/modules/text_editor)
- [Text Input](/latest/reference/modules/text_input)
- [Theme](/latest/reference/modules/theme)
- [Toggler](/latest/reference/modules/toggler)
- [Tooltip](/latest/reference/modules/tooltip)
- [Vertical Slider](/latest/reference/modules/vertical_slider)

## Related

- [Families](/latest/reference/families)
- [Constructors](/latest/reference/constructors)

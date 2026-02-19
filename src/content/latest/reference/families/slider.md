---
title: Family - Slider
description: Unified reference for the Slider widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 738
---

# Family - Slider

This page unifies related iced::widget APIs for the **Slider** family.

## API surfaces

- Module: [iced::widget::slider](/latest/reference/modules/slider)
- Constructor: [iced::widget::slider](/latest/reference/constructors/slider)
- Element: [iced::widget::Slider](/latest/reference/elements/slider)

## Surface summaries

### Module

Sliders let users set a value by moving an indicator.

### Constructor

Creates a new
Slider
.

### Element

An horizontal bar and a handle that selects a single value from a range of
values.

## Verified constructor signature

```rust
pub fn slider<'a, T, Message, Theme>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
    Theme: Catalog + 'a,
```

## Verified element declaration

```rust
pub struct Slider<'a, T, Message, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/toast/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/sandpiles/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/integration/src/controls.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> Element<'_, Message> {
    slider(0.0..=100.0, state.value, Message::ValueChanged).into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ValueChanged(value) => {
            state.value = value;
        }
    }
}
```

### Element example

```rust
use iced::widget::slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> Element<'_, Message> {
    slider(0.0..=100.0, state.value, Message::ValueChanged).into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ValueChanged(value) => {
            state.value = value;
        }
    }
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want one page that links module, constructor, and element surfaces.
- You are deciding which API surface to start from.
- You need a practical map for this widget domain.

## Minimal example

```rust
// Typical flow:
// 1) Start with constructor usage.
// 2) Move to module docs for style/state details.
// 3) Use element docs for type-level control.
```

## How it works

Family pages connect related docs so you do not miss capabilities that are split across constructor/module/element pages.

## Common patterns

```rust
// Build with constructor APIs first,
// then refine behavior/styles through related module and element docs.
```

## Gotchas / tips

- Family routes normalize naming; module/function/struct names may differ slightly.
- Prefer this page as your entrypoint when learning unfamiliar widgets.
- Follow example references here before inventing integration patterns.

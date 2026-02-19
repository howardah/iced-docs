---
title: Constructor - Progress Bar
description: Function reference for iced::widget::progress_bar.
version: latest
last_updated: 2026-02-19
order: 322
---

# Constructor - Progress Bar

Authoritative source: `ref/doc/iced/widget/fn.progress_bar.html`.

## Rustdoc summary

Creates a new
ProgressBar
.

## Verified signature

```rust
pub fn progress_bar<'a, Theme>(
    range: RangeInclusive<f32>,
    value: f32,
) -> ProgressBar<'a, Theme>
where
    Theme: Catalog + 'a,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/progress_bar/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/scrollable/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::progress_bar;

struct State {
   progress: f32,
}

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    progress_bar(0.0..=100.0, state.progress).into()
}
```

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

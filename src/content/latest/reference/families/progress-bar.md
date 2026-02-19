---
title: Family - Progress Bar
description: Unified reference for the Progress Bar widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 727
---

# Family - Progress Bar

This page unifies related iced::widget APIs for the **Progress Bar** family.

## API surfaces

- Module: [iced::widget::progress_bar](/latest/reference/modules/progress_bar)
- Constructor: [iced::widget::progress_bar](/latest/reference/constructors/progress_bar)
- Element: [iced::widget::ProgressBar](/latest/reference/elements/progress-bar)

## Surface summaries

### Module

Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.

### Constructor

Creates a new
ProgressBar
.

### Element

A bar that displays progress.

## Verified constructor signature

```rust
pub fn progress_bar<'a, Theme>(
    range: RangeInclusive<f32>,
    value: f32,
) -> ProgressBar<'a, Theme>
where
    Theme: Catalog + 'a,
```

## Verified element declaration

```rust
pub struct ProgressBar<'a, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/progress_bar/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/scrollable/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

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

### Element example

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

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

---
title: Element - Progress Bar
description: Struct reference for iced::widget::ProgressBar.
version: latest
last_updated: 2026-02-19
order: 514
---

# Element - Progress Bar

Authoritative source: ref/doc/iced/widget/struct.ProgressBar.html.

## Rustdoc summary

A bar that displays progress.

## Verified type declaration

```rust
pub struct ProgressBar<'a, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/progress_bar/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/scrollable/src/main.rs
- ref/examples/styling/src/main.rs

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

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

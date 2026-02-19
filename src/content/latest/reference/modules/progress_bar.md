---
title: Module - Progress Bar
description: Module-level reference for iced::widget::progress_bar.
version: latest
last_updated: 2026-02-19
order: 112
---

# Module - Progress Bar

Authoritative source: `ref/doc/iced/widget/progress_bar/index.html`.

## Rustdoc description

Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::progress_bar.

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

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

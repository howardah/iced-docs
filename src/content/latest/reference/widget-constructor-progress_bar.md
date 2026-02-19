---
title: Widget Constructor - progress_bar
description: Function reference for iced::widget::progress_bar.
version: latest
last_updated: 2026-02-19
order: 322
---

# Widget Constructor - iced::widget::progress_bar

Authoritative source: ref/doc/iced/widget/fn.progress_bar.html.

## Rustdoc summary

Creates a new
ProgressBar
.

## Verified signature

```rust
pub fn progress_bar<'a, Theme>(
range: RangeInclusive<f32>,
value: f32,
) -> ProgressBar<'a, Theme>where
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

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

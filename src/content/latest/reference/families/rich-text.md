---
title: Family - Rich Text
description: Unified reference for the Rich Text widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 730
---

# Family - Rich Text

This page unifies related iced::widget APIs for the **Rich Text** family.

## API surfaces

- Constructor: [iced::widget::rich_text](/latest/reference/constructors/rich_text)

## Surface summaries

### Constructor

Creates a new
Rich
text widget with the provided spans.

## Verified constructor signature

```rust
pub fn rich_text<'a, Link, Message, Theme, Renderer>(
    spans: impl AsRef<[Span<'a, Link, <Renderer as Renderer>::Font>]> + 'a,
) -> Rich<'a, Link, Message, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog + 'a,
    Renderer: Renderer,
    <Renderer as Renderer>::Font: 'a,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/gallery/src/civitai.rs
- ref/examples/vectorial_text/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::font;
use iced::widget::{rich_text, span};
use iced::{color, never, Font};

#[derive(Debug, Clone)]
enum Message {
    LinkClicked(&'static str),
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    rich_text([
        span("I am red!").color(color!(0xff0000)),
        span(" "),
        span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
    ])
    .on_link_click(never)
    .size(20)
    .into()
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

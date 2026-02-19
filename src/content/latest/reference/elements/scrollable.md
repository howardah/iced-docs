---
title: Element - Scrollable
description: Struct reference for iced::widget::Scrollable.
version: latest
last_updated: 2026-02-19
order: 519
---

# Element - Scrollable

Authoritative source: ref/doc/iced/widget/struct.Scrollable.html.

## Rustdoc summary

A widget that can vertically display an infinite amount of content with a
scrollbar.

## Verified type declaration

```rust
pub struct Scrollable<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/tour/src/main.rs
- ref/examples/scrollable/src/main.rs
- ref/examples/changelog/src/main.rs
- ref/examples/combo_box/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/gallery/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{column, scrollable, space};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    scrollable(column![
        "Scroll me!",
        space().height(3000),
        "You did it!",
    ]).into()
}
```

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need the concrete widget struct type in signatures.
- You are debugging type errors involving generic bounds.
- You want lower-level control than constructor-only docs provide.

## Minimal example

```rust
// Constructors usually produce this element type.
// Name the type explicitly only when type-level APIs need it.
```

## How it works

Element structs are the underlying widget types used by constructors. Most app code can stay constructor-first, then use element docs for advanced typing/customization.

## Common patterns

```rust
// Use constructors in normal UI code,
// and reserve explicit element types for reusable abstractions.
```

## Gotchas / tips

- You usually do not need to construct element structs directly.
- Read trait bounds carefully when adding custom renderer/theme types.
- If a method is missing, check the related module page.

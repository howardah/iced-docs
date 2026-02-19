---
title: Element - Text Input
description: Struct reference for iced::widget::TextInput.
version: latest
last_updated: 2026-02-19
order: 526
---

# Element - Text Input

Authoritative source: ref/doc/iced/widget/struct.TextInput.html.

## Rustdoc summary

A field that can be filled with text.

## Verified type declaration

```rust
pub struct TextInput<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
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
- ref/examples/changelog/src/main.rs
- ref/examples/integration/src/controls.rs
- ref/examples/lazy/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/multi_window/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::text_input;

struct State {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String)
}

fn view(state: &State) -> Element<'_, Message> {
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
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

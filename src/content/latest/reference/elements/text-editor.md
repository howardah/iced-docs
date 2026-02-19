---
title: Element - Text Editor
description: Struct reference for iced::widget::TextEditor.
version: latest
last_updated: 2026-02-19
order: 525
---

# Element - Text Editor

Authoritative source: ref/doc/iced/widget/struct.TextEditor.html.

## Rustdoc summary

A multi-line text input.

## Verified type declaration

```rust
pub struct TextEditor<'a, Highlighter, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Highlighter: Highlighter,
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::text_editor;

struct State {
   content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action)
}

fn view(state: &State) -> Element<'_, Message> {
    text_editor(&state.content)
        .placeholder("Type something here...")
        .on_action(Message::Edit)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
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

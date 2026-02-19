---
title: Constructor - Text Editor
description: Function reference for iced::widget::text_editor.
version: latest
last_updated: 2026-02-19
order: 338
---

# Constructor - Text Editor

Authoritative source: `ref/doc/iced/widget/fn.text_editor.html`.

## Rustdoc summary

Creates a new
TextEditor
.

## Verified signature

```rust
pub fn text_editor<'a, Message, Theme, Renderer>(
    content: &'a Content<Renderer>,
) -> TextEditor<'a, PlainText, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/markdown/src/main.rs
- ref/examples/editor/src/main.rs

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

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

## Use this when...

- You want the canonical entrypoint for creating this widget/helper.
- You need concrete constructor arguments and builder chaining.
- You are wiring UI interactions into typed messages.

## Minimal example

```rust
// Call this constructor in `view`, then map events to Message variants.
```

## How it works

Constructors return typed widget values. You configure behavior via builder methods and emit `Message` values for `update` to handle.

## Common patterns

```rust
// Keep constructor calls close to their message mapping.
// Prefer small helper functions for repeated widget setups.
```

## Gotchas / tips

- Re-check argument order in the verified signature on this page.
- Keep side effects out of `view`; trigger them from `update` with Task.
- Use the related family page when deciding between module/element APIs.

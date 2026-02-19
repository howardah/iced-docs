---
title: Module - Text Editor
description: Module-level reference for iced::widget::text_editor.
version: latest
last_updated: 2026-02-19
order: 123
---

# Module - Text Editor

Authoritative source: `ref/doc/iced/widget/text_editor/index.html`.

## Rustdoc description

Text editors display a multi-line text input for text editing.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::text_editor.

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

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)

## Use this when...

- You need module-level APIs beyond the basic constructor call.
- You want family-specific style/state traits and helper types.
- You are building reusable widget abstractions.

## Minimal example

```rust
// Start with the constructor from this module family in `view`.
// Then move to module APIs for deeper customization.
```

## How it works

Module docs explain the namespace that groups constructors, types, and related traits. In everyday app code, this helps you discover advanced options after basic usage works.

## Common patterns

```rust
// Message flow pattern:
// widget interaction -> Message -> update -> state change -> rerender
```

## Gotchas / tips

- Check this page together with its family page for complete context.
- Verify trait bounds and associated types in rustdoc when custom styling fails.
- Keep module imports explicit while learning.

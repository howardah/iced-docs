---
title: Family - Text Editor
description: Unified reference for the Text Editor widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 744
---

# Family - Text Editor

This page unifies related iced::widget APIs for the **Text Editor** family.

## API surfaces

- Module: [iced::widget::text_editor](/latest/reference/modules/text_editor)
- Constructor: [iced::widget::text_editor](/latest/reference/constructors/text_editor)
- Element: [iced::widget::TextEditor](/latest/reference/elements/text-editor)

## Surface summaries

### Module

Text editors display a multi-line text input for text editing.

### Constructor

Creates a new
TextEditor
.

### Element

A multi-line text input.

## Verified constructor signature

```rust
pub fn text_editor<'a, Message, Theme, Renderer>(
    content: &'a Content<Renderer>,
) -> TextEditor<'a, PlainText, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct TextEditor<'a, Highlighter, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Highlighter: Highlighter,
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

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

### Element example

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

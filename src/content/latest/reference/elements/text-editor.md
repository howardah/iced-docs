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
pub struct TextEditor<'a, Highlighter, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>where
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

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)

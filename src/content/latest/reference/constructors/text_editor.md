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

- ref/examples/editor/src/main.rs
- ref/examples/markdown/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

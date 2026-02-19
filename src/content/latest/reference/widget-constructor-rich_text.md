---
title: Widget Constructor - rich_text
description: Function reference for iced::widget::rich_text.
version: latest
last_updated: 2026-02-19
order: 325
---

# Widget Constructor - iced::widget::rich_text

Authoritative source: ref/doc/iced/widget/fn.rich_text.html.

## Rustdoc summary

Creates a new
Rich
text widget with the provided spans.

## Verified signature

```rust
pub fn rich_text<'a, Link, Message, Theme, Renderer>(
spans: impl AsRef<[Span<'a, Link, <Renderer as Renderer>::Font>]> + 'a,
) -> Rich<'a, Link, Message, Theme, Renderer>where
Link: Clone + 'static,
Theme: Catalog + 'a,
Renderer: Renderer,
<Renderer as Renderer>::Font: 'a,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)

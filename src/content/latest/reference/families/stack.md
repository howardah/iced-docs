---
title: Family - Stack
description: Unified reference for the Stack widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 741
---

# Family - Stack

This page unifies related iced::widget APIs for the **Stack** family.

## API surfaces

- Constructor: [iced::widget::stack](/latest/reference/constructors/stack)
- Element: [iced::widget::Stack](/latest/reference/elements/stack)

## Surface summaries

### Constructor

Creates a new
Stack
with the given children.

### Element

Creates a new
Text
widget with the provided content.

## Verified constructor signature

```rust
pub fn stack<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Stack<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Stack<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)

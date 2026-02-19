---
title: Element - Rule
description: Struct reference for iced::widget::Rule.
version: latest
last_updated: 2026-02-19
order: 518
---

# Element - Rule

Authoritative source: ref/doc/iced/widget/struct.Rule.html.

## Rustdoc summary

Display a horizontal or vertical rule for dividing content.

## Verified type declaration

```rust
pub struct Rule<'a, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- TODO(api-verify): add canonical example mapping for this item.

## Inline Examples (from rustdoc)

```rust
use iced::widget::rule;

#[derive(Clone)]
enum Message {
    // ...,
}

fn view(state: &State) -> Element<'_, Message> {
    rule::horizontal(2).into()
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

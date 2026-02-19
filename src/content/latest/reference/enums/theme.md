---
title: Enum - Theme
description: Enum reference for iced::Theme.
version: latest
last_updated: 2026-02-19
order: 80
---

# Enum - Theme

Authoritative source: `ref/doc/iced/enum.Theme.html`.

## Rustdoc summary

A built-in theme.

## Verified declaration

```rust
pub enum Theme {
Show 23 variants    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
    Custom(Arc<Custom>),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/clock/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/arc/src/main.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)

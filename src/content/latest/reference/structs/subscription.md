---
title: Struct - Subscription
description: Struct reference for iced::Subscription.
version: latest
last_updated: 2026-02-19
order: 95
---

# Struct - Subscription

Authoritative source: `ref/doc/iced/struct.Subscription.html`.

## Rustdoc summary

A request to listen to external events.

## Verified declaration

```rust
pub struct Subscription<T> { /* private fields */ }
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/clock/src/main.rs
- ref/examples/sandpiles/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/arc/src/main.rs
- ref/examples/gallery/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::time::{self, Duration, Instant};
use iced::Subscription;

struct State {
    timer_enabled: bool,
}

fn subscription(state: &State) -> Subscription<Instant> {
    if state.timer_enabled {
        time::every(Duration::from_secs(1))
    } else {
        Subscription::none()
    }
}
```

```rust
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::Stream;
use iced::stream;
use iced::Subscription;

pub enum Event {
    Ready(mpsc::Sender<Input>),
    WorkFinished,
    // ...
}

enum Input {
    DoSomeWork,
    // ...
}

fn some_worker() -> impl Stream<Item = Event> {
    stream::channel(100, async |mut output| {
        // Create channel
        let (sender, mut receiver) = mpsc::channel(100);

        // Send the sender back to the application
        output.send(Event::Ready(sender)).await;

        loop {
            use iced_futures::futures::StreamExt;

            // Read next input sent from `Application`
            let input = receiver.select_next_some().await;

            match input {
                Input::DoSomeWork => {
                    // Do some async work...

                    // Finally, we can optionally produce a message to tell the
                    // `Application` the work is done
                    output.send(Event::WorkFinished).await;
                }
            }
        }
    })
}

fn subscription() -> Subscription<Event> {
    Subscription::run(some_worker)
}
```

## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)

## Use this when...

- You need this concrete Iced type in state/configuration/helpers.
- You want stronger typing than primitive values provide.
- You are working with runtime primitives like Task/Subscription/Settings.

## Minimal example

```rust
// Construct and pass this struct where the corresponding API expects it.
```

## How it works

Crate-level structs define shared runtime, geometry, styling, and configuration data. Using them directly keeps app code aligned with rustdoc contracts.

## Common patterns

```rust
// Centralize commonly reused struct values in helper constructors.
```

## Gotchas / tips

- Prefer explicit Iced structs over loosely typed primitives where possible.
- Check trait bounds when using these types in generic code.
- For runtime structs, keep lifecycle ownership clear.

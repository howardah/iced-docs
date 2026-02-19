# AGENTS.md

This repository contains a Dioxus-based documentation site for `iced`.

This project uses **Dioxus 0.7.x**.
All code MUST follow the Dioxus 0.7 API.

> `cx`, `Scope`, and `use_state` DO NOT exist in 0.7  
> Use `use_signal`, `use_memo`, `use_resource`, and Signals instead.

---

# 📘 Primary Specification

All architectural, structural, and documentation decisions MUST follow:

**SITE_SPEC.md**

If there is a conflict between this file and SITE_SPEC.md,
**SITE_SPEC.md takes precedence**.

---

# 📁 Repository Layout (Required)

The site is a single Rust crate at the repository root.
The Dioxus site code and the documentation content live under `/src`.

```
/Cargo.toml
/src
  /app                # Dioxus UI + routing + page templates
  /content            # Markdown docs (ingested by the site)
  /public             # Static assets served by the site
  /docs-meta          # Optional metadata helpers
  main.rs
/scripts              # Validation/build tooling
/ref                  # Agent-only reference (usually gitignored)
  /example            # Official iced examples (read-only)
  /doc                # Rustdoc output (authoritative API)
  /rust/doc/iced      # Alternative rustdoc location
SITE_SPEC.md
AGENTS.md
```

Rules:
- `/src/content` is the only doc content the site renders.
- `/ref` is NOT required to run the site; it exists to guide doc generation accuracy.
- `/ref` must be treated as read-only and must not be modified.

---

# 🔒 Authoritative Sources Policy (Non-negotiable)

When writing or updating documentation content:

1) Verify API definitions in `ref/doc` (or `ref/rust/doc/iced`)  
2) Cross-check usage patterns in `ref/example`  
3) Only then write prose + snippets

Never:
- invent APIs
- guess signatures
- fabricate behavior

If uncertain:
- add `TODO(api-verify)`
- do not hallucinate details

Rustdoc overrides example inference if they differ.

---

# ⚙️ Dioxus 0.7 Quick Reference

## Dependency

```toml
[dependencies]
dioxus = { version = "0.7.1" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

## Launching

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Hello, Dioxus!" }
}
```

Serve with:

```sh
dx serve
```

---

# 🧩 Component Rules

- Components use `#[component]`
- Props must be owned types
- Props must implement `PartialEq + Clone`
- State uses `use_signal`
- Expensive derived values use `use_memo`
- Async uses `use_resource`
- Context via `use_context_provider` / `use_context`

Never use pre-0.7 Dioxus APIs.

---

# 🗂 Routing

Use enum-based routing with `#[derive(Routable)]`.
Layouts use `#[layout(LayoutComponent)]` and render `Outlet<Route>`.

Router entry point:

```rust
rsx! { Router::<Route> {} }
```

Feature flag:

```toml
dioxus = { version = "0.7.1", features = ["router"] }
```

---

# 🔎 UX Requirements (From SITE_SPEC.md)

- Sidebar navigation generated from `/src/content`
- Version selector dropdown
- On-page TOC for h2/h3
- Client-side search index
- Deep-linkable headings
- Code copy buttons

---

# 🚦 CI Quality Gates

Build must fail if:
- internal links are broken
- required frontmatter missing
- search index build fails
- build warnings exceed threshold

---

# 🎯 Agent Behavior Expectations

- Follow SITE_SPEC.md strictly
- Prefer correctness over completeness
- Prefer TODO over guessing
- Keep rendering logic in `/src/app`, content in `/src/content`

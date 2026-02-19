# Mission: Iced Documentation Website (Dioxus)

## Problem statement
Rust GUI library `iced` is powerful but fragmented and inconsistently documented.
Build a modern, fast, searchable documentation website using **Dioxus** that provides:

1) **Guide** — installation + setup + tooling + bundling + distribution  
2) **Reference** — detailed but approachable documentation of widgets, core types, and important APIs  
3) **Tutorial** — brief, practical walkthroughs tying everything together  

This site must be structured, maintainable, version-aware, and generated from consistent content patterns.

---

# 🔒 Authoritative Sources (CRITICAL)

The documentation site MUST treat the following as the source of truth:

- `ref/example/`  
  Official iced examples (not committed to the repo).  
  These are canonical usage patterns and behavioral references.

- `ref/doc/` and/or `ref/rust/doc/iced/`  
  Generated Rustdocs for iced.  
  These are the authoritative API definitions.

## Hard rules

- DO NOT invent API signatures.
- DO NOT guess struct fields or trait bounds.
- DO NOT fabricate features or behavior.
- If uncertain, inspect:
  - `ref/doc/` (rustdoc)
  - `ref/example/` (real usage patterns)
- If ambiguity remains:
  - Mark with `TODO(api-verify)`
  - Do not hallucinate details.

The generated documentation must be **derived from and consistent with** those sources.

> Note: `/ref` exists to help the coding agent generate accurate docs.  
> `/ref` is **not** required to run or ship the site, and should typically be gitignored.

---

# Non-goals

- Maintaining iced itself
- Rewriting or modifying official examples
- Hosting infrastructure beyond static build output
- Exhaustively documenting every third-party ecosystem crate

---

# Target outcomes (Definition of Done)

- Fully working Dioxus documentation site
- Mobile + desktop responsive
- Functional search
- Version-aware
- No broken internal links
- All Reference pages reflect actual iced API from rustdoc
- No API information contradicts rustdoc

---

# 📁 Directory Structure (Reconsidered)

Rationale:
- `/content` is runtime input the site ingests (Markdown + frontmatter), so it should live alongside the site code.
- `/ref` is **agent-only** truth data and does not need to be part of the crate build output.

Recommended layout (single crate at repo root):

```
/Cargo.toml
/src
  /app                # Dioxus UI + routing + page templates
  /content            # Markdown docs to ingest at runtime/build time
    /latest
      /guide
      /reference
      /tutorial
    /<version>
  /public             # Static assets served by the site (images, icons, css)
  /docs-meta          # Build/version metadata helpers (optional, may be /scripts too)
  /lib.rs (optional)
  /main.rs
/scripts              # Validation/build tooling (link check, frontmatter validation, search index build)
/ref                  # Agent-only reference (usually gitignored)
  /example            # Official iced examples (read-only reference)
  /doc                # Rustdoc output (authoritative API source)
  /rust/doc/iced      # Alternative rustdoc location
SITE_SPEC.md
AGENTS.md
```

Rules:
- `/src/content` is the **only** authoritative content the site renders.
- `/src/app` contains rendering logic only; avoid hardcoding doc text in Rust.
- `/src/public` contains static assets referenced by Markdown via stable paths.
- `/ref` must never be required for `cargo run` or for CI builds that verify the site UI.
- Versioned content lives under `/src/content/<version>/`; `latest` must alias the newest documented iced version.

---

# Information Architecture (IA)

## 1) Guide
Goal: Take a developer from zero → shipping.

Minimum pages:
- Overview
- Installation (toolchain, platform notes, feature flags)
- Project Setup (scaffold, structure)
- Tooling (build loop, debugging, logging/tracing)
- Bundling (release builds, assets)
- Distribution (Win/macOS/Linux notes, pitfalls checklist)

Guide may synthesize from examples, but must not contradict rustdoc.

## 2) Reference
Goal: “I need widget X or type Y — show me everything.”

Organize by:
- Core concepts
- Widgets / Elements
- Important Types / Traits
- Styling / Theming
- Layout
- Input & Events
- Commands / Subscriptions (if present in current API)

Each widget/type page MUST include:
- Overview (what it is)
- When to use it
- Constructors / builder methods (verified via rustdoc)
- Important methods
- Message/output behavior
- Styling hooks (if supported)
- Common pitfalls
- Related APIs

All API surfaces must be verified against rustdoc (`ref/doc` or `ref/rust/doc/iced`).
Inline examples should be minimal and derived from patterns in `ref/example` (not invented).

## 3) Tutorial
Goal: Short, realistic walkthroughs.

Tutorials may synthesize patterns from `ref/example`,
but must not copy large example files verbatim.

Progression:
- Tutorial 1: Basic window + button
- Tutorial 2: Layout + input
- Tutorial 3: Async / Command pattern
- Tutorial 4: Theming & reusable components

Each tutorial ends with:
- What you learned
- Links to relevant Reference pages

---

# Content Requirements (Hard Rules)

Every page MUST include:
- Title
- Description (SEO summary)
- Version
- Last updated metadata

All code blocks MUST:
- Be language-tagged
- Include necessary imports when ambiguity exists
- Reflect actual API signatures from rustdoc

Never write:
- “You can call X like this…” unless verified
- “This returns Y” unless verified

If unsure:
- Add `TODO(api-verify)`
- Do not fabricate

---

# Site Features

## Navigation
- Sidebar tree: Guide / Reference / Tutorial (generated from `/src/content`)
- Version selector (dropdown)
- GitHub link
- On-page TOC (h2/h3)
- Prev/Next navigation
- Deep-linkable headings
- Copy buttons on code blocks

## Search
- Client-side
- Index page titles, headings, body
- Highlight matches
- Keyboard navigation
- Lazy-load index if large

## Rendering
- Markdown + frontmatter
- Static generation preferred (or prebuilt content index)
- Syntax highlighting
- Clean typography
- Light + dark mode

---

# Quality Gates (CI must fail if)

- Internal links are broken
- Required frontmatter missing
- Search index fails
- Build warnings exceed threshold
- Any “verified API” section contains unverified signatures (if you implement automated checks)

---

# Accuracy Policy

When documenting an API:
1) Inspect rustdoc first (`ref/doc` or `ref/rust/doc/iced`)
2) Cross-reference usage in `ref/example`
3) Then write explanatory prose

If behavior differs between example and rustdoc:
- Prefer rustdoc
- Add a clarification note

Never reverse-engineer behavior from guesswork.

---

# Agent Execution Plan

1) Build Dioxus app + routing
2) Implement Markdown loader + frontmatter schema (reading from `/src/content`)
3) Generate sidebar tree from `/src/content` structure
4) Implement version switching
5) Implement search
6) Build page templates
7) Seed Guide pages
8) Generate initial Reference pages using rustdoc inspection
9) Cross-check against examples in `ref/example`
10) Add CI validation scripts
11) Polish styling and a11y

---

# Acceptance Criteria

- Site builds without warnings
- Search works
- Version switching works
- All Reference API signatures verified
- No hallucinated APIs
- No broken links
- Responsive + accessible navigation

---

# Decision Heuristics

- Prefer simplicity
- Prefer correctness over completeness
- Prefer marking TODO over guessing
- Prefer rustdoc over example inference

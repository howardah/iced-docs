# Mission: Iced Documentation Website (Dioxus)

## Problem statement

Rust GUI library `iced` is powerful but fragmented and inconsistently
documented. Build a modern, fast, searchable documentation website using
**Dioxus** that provides:

1)  **Guide** --- installation + setup + tooling + bundling +
    distribution\
2)  **Reference** --- detailed but approachable documentation of
    widgets, core types, and important APIs\
3)  **Tutorial** --- brief, practical walkthroughs tying everything
    together

This site must be structured, maintainable, version-aware, and generated
from consistent content patterns.

------------------------------------------------------------------------

# 🔒 Authoritative Sources (CRITICAL)

The documentation site MUST treat the following as the source of truth:

-   `ref/example/`\
    Official iced examples (not committed to the repo).\
    These are canonical usage patterns and behavioral references.

-   `ref/doc/` and/or `rust/doc/iced/`\
    Generated Rustdocs for iced.\
    These are the authoritative API definitions.

## Hard rules

-   DO NOT invent API signatures.
-   DO NOT guess struct fields or trait bounds.
-   DO NOT fabricate features or behavior.
-   If uncertain, inspect:
    -   `ref/doc/` (rustdoc)
    -   `ref/example/` (real usage patterns)
-   If ambiguity remains:
    -   Mark with `TODO(api-verify)`
    -   Do not hallucinate details.

The generated documentation must be **derived from and consistent with**
those sources.

------------------------------------------------------------------------

# Non-goals

-   Maintaining iced itself
-   Rewriting or modifying official examples
-   Hosting infrastructure beyond static build output
-   Exhaustively documenting every third-party ecosystem crate

------------------------------------------------------------------------

# Target outcomes (Definition of Done)

-   Fully working Dioxus documentation site
-   Mobile + desktop responsive
-   Functional search
-   Version-aware
-   No broken internal links
-   All Reference pages reflect actual iced API from rustdoc
-   No API information contradicts rustdoc

------------------------------------------------------------------------

# Information Architecture (IA)

## 1) Guide

Goal: Take a developer from zero → shipping.

Minimum pages:

-   Overview
-   Installation
    -   Rust toolchain requirements
    -   Platform notes
    -   Feature flags overview
-   Project Setup
    -   Minimal scaffold
    -   Project structure recommendations
-   Tooling
    -   Build workflows
    -   Debugging
    -   Logging / tracing
-   Bundling
    -   Release builds
    -   Assets
-   Distribution
    -   Windows notes
    -   macOS notes
    -   Linux notes
    -   Pitfalls checklist

Guide content may interpret and synthesize from examples, but must not
contradict rustdoc.

------------------------------------------------------------------------

## 2) Reference

Goal: "I need widget X or type Y --- show me everything."

Organize by:

-   Core concepts
-   Widgets / Elements
-   Important Types / Traits
-   Styling / Theming
-   Layout
-   Input & Events
-   Commands / Subscriptions (if present in current API)

Each widget/type page MUST include:

-   Overview (what it is)
-   When to use it
-   Constructors / builder methods (verified via rustdoc)
-   Important methods
-   Message/output behavior
-   Styling hooks (if supported)
-   Common pitfalls
-   Related APIs

All API surfaces must be verified against: - `ref/doc` - or
`rust/doc/iced`

Examples shown inline should be: - Derived from patterns in
`ref/example` - Minimal and accurate - Not invented

If a method exists in rustdoc but is deprecated or unstable, it must be
labeled clearly.

------------------------------------------------------------------------

## 3) Tutorial

Goal: Short, realistic walkthroughs.

Tutorials may synthesize patterns from `ref/example`, but must not copy
large example files verbatim.

Tutorial progression:

-   Tutorial 1: Basic window + button
-   Tutorial 2: Layout + input
-   Tutorial 3: Async / Command pattern
-   Tutorial 4: Theming & reusable components

Each tutorial ends with: - "What you learned" - Links to relevant
Reference pages

------------------------------------------------------------------------

# Content Requirements (Hard Rules)

Every page MUST include:

-   Title
-   Description (SEO summary)
-   Version
-   Last updated metadata

All code blocks MUST:

-   Be language-tagged
-   Include necessary imports when ambiguity exists
-   Reflect actual API signatures from rustdoc

Never write: - "You can call X like this..." unless verified - "This
returns Y" unless verified

If unsure: - Add `TODO(api-verify)` - Do not fabricate

------------------------------------------------------------------------

# Site Features

## Navigation

-   Sidebar tree: Guide / Reference / Tutorial
-   Version selector (dropdown)
-   GitHub link
-   On-page TOC (h2/h3)
-   Prev/Next navigation
-   Deep-linkable headings
-   Copy buttons on code blocks

## Search

-   Client-side
-   Index page titles, headings, body
-   Highlight matches
-   Keyboard navigation
-   Lazy-load index if large

## Rendering

-   Markdown + frontmatter
-   Static generation
-   Syntax highlighting
-   Clean typography
-   Light + dark mode

------------------------------------------------------------------------

# Versioning Model

Content stored under:

/content/`<version>`{=html}/ /content/latest/

Version switcher updates base path.

"latest" is alias of newest version.

Version must match iced version documented in `ref/doc`.

------------------------------------------------------------------------

# Repository Structure

/app Dioxus site /content /latest /guide /reference /tutorial
/`<version>`{=html} /ref /example official examples (read-only
reference) /doc rustdoc output /rust/doc/iced alternative rustdoc
location /scripts /public /docs-meta

`/ref` is read-only input for documentation generation. It is NOT to be
modified.

------------------------------------------------------------------------

# Page Templates

Implement:

-   GuidePage
-   ReferencePage
-   TutorialPage

ReferencePage enforces sections:

-   Overview
-   API
-   Examples
-   Pitfalls
-   Related

Templates must auto-generate TOC and metadata.

------------------------------------------------------------------------

# Styling Requirements

-   Clean, modern design
-   Design tokens (spacing, typography, colors)
-   Light + dark mode
-   Responsive layout
-   Callouts: Note / Tip / Warning
-   Consistent spacing
-   No ad-hoc styling values

------------------------------------------------------------------------

# Quality Gates (CI must fail if)

-   Internal links broken
-   Required frontmatter missing
-   Rustdoc mismatch detected (if automated comparison implemented)
-   Search index fails
-   Build warnings exceed threshold

------------------------------------------------------------------------

# Accuracy Policy

When documenting an API:

1)  Inspect rustdoc first (`ref/doc`)
2)  Cross-reference real usage in `ref/example`
3)  Only then write explanatory prose

If behavior differs between example and rustdoc: - Prefer rustdoc - Add
clarification note

Never reverse-engineer behavior from guesswork.

------------------------------------------------------------------------

# Agent Execution Plan

1)  Scaffold Dioxus app + routing
2)  Implement Markdown loader + frontmatter schema
3)  Implement sidebar tree from content structure
4)  Implement version switching
5)  Implement search
6)  Build page templates
7)  Seed Guide pages
8)  Generate initial Reference pages using rustdoc inspection
9)  Cross-check against examples in `ref/example`
10) Add CI validation scripts
11) Polish styling and a11y

------------------------------------------------------------------------

# Acceptance Criteria

-   Site builds without warnings
-   Search works
-   Version switching works
-   All Reference API signatures verified
-   No hallucinated APIs
-   No broken links
-   Responsive + accessible navigation

------------------------------------------------------------------------

# Decision Heuristics

-   Prefer simplicity
-   Prefer correctness over completeness
-   Prefer marking TODO over guessing
-   Prefer rustdoc over example inference

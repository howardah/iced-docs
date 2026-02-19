# Mission: Iced Documentation Website (Dioxus)

## Problem statement
Rust GUI library `iced` is powerful but has fragmented / insufficient documentation. Build a modern, fast, searchable documentation website using **Dioxus** that provides:
1) **Guide**: installation + setup + tooling + bundling + distribution
2) **Reference**: approachable but detailed reference for widgets/elements + key APIs
3) **Tutorial**: short end-to-end tutorials showing how to build real apps

This site must be maintainable, versioned, and generate consistent pages from structured data.

---

## Non-goals (explicitly out of scope)
- Writing/maintaining iced itself
- Hosting/DevOps beyond producing a static build output
- Covering every third-party crate in the iced ecosystem (only where directly necessary)
- Publishing opinions as facts (flag “Best practice” vs “Must”)

---

## Target outcomes (what “done” means)
- A Dioxus-based documentation site that:
  - Builds successfully in CI (no manual steps)
  - Renders cleanly on mobile + desktop
  - Has working navigation, search, and deep links for all pages
  - Supports multi-version docs (at least “latest” and one previous version)
  - Has no dead links (internal link checking in CI)
  - Has a consistent content format that can be expanded over time

---

## Information architecture (IA)
Top-level sections:

### 1) Guide
Goal: get a developer from “no setup” to “shipping”.
Pages (minimum set):
- Overview: what iced is, major concepts, what this site covers
- Installation:
  - Rust toolchain prerequisites
  - Choosing iced version/features
  - Platform notes (Windows/macOS/Linux)
- Project setup:
  - Minimal example scaffold
  - Recommended project layout
  - Feature flags explained (what they do, when to enable)
- Tooling:
  - hot reload/dev loop if applicable
  - debugging tips
  - logging, tracing
  - formatting/lints (rustfmt/clippy)
- Bundling:
  - building release binaries
  - assets (icons, fonts, images)
  - configuration
- Distribution:
  - Windows packaging notes
  - macOS signing/notarization notes (high-level)
  - Linux packaging notes
  - common pitfalls checklist

### 2) Reference
Goal: “I need to use widget X or API Y—show me everything quickly, with examples.”
Organize by:
- Widgets / Elements (each widget has its own page)
- Core Concepts
- Important Types / Functions / Traits
- Styling & theming
- Layout & responsiveness
- Input & events
- State management patterns (approachable, not dogmatic)

Reference must be generated from **structured content** (frontmatter + consistent sections), so pages are uniform.

Minimum widget coverage (starter list; expand as needed):
- Application / Program / Command / Subscription (if applicable to current iced API)
- Element, Widget, Renderer basics
- Layout primitives: Column, Row, Container, Space
- Inputs: Button, TextInput, Checkbox, Toggler, Slider, PickList
- Display: Text, Image, Rule, ProgressBar
- Scrolling: Scrollable
- Overlay / modal patterns if provided by iced
- Canvas if relevant

Each reference page must include:
- What it is (1–2 paragraphs)
- When to use it (bullets)
- Key properties/constructors (table or list)
- Events/messages produced
- Styling / theming hooks
- Performance notes (if any)
- “Common pitfalls” section
- 1–3 runnable examples (small, focused)
- Related links (internal)

### 3) Tutorial
Goal: “put it all together.”
Tutorials should be short, practical, and build on each other:
- Tutorial 1: “Hello Iced” (window + text + button)
- Tutorial 2: layout + input (form)
- Tutorial 3: async (fetch or timer) + Command/Subscription pattern
- Tutorial 4: theming + reusable components
Each tutorial ends with:
- “What you learned”
- “Next steps” links to Reference pages used

---

## Content requirements (hard rules)
- Every page MUST have:
  - Title
  - Short description (for SEO + page header)
  - Breadcrumbs (or nav context)
  - Last updated + version (from build metadata)
- All code blocks MUST:
  - Be tagged with language (rust, toml, bash, etc.)
  - Prefer minimal, compilable snippets
  - Include imports when ambiguity is likely
- Avoid vague claims:
  - Separate “Facts” from “Recommendations”
  - If a statement depends on iced version/platform, label it
- Write in a friendly, direct tone:
  - “Do X” / “Avoid Y” with reasons
  - Short paragraphs; use lists

---

## Site features (engineering requirements)
### Navigation & UX
- Left sidebar with expandable tree:
  - Guide / Reference / Tutorial
- Top bar:
  - search input
  - version selector (dropdown)
  - GitHub link
- Right-side “On this page” TOC for long pages (headings h2/h3)
- Prev/Next links within a section
- Copy button on code blocks
- Deep-linkable headings (anchor links)

### Search
- Must work fully client-side (static hosting friendly).
- Index headings + page titles + body text.
- Support:
  - quick results while typing
  - keyboard navigation
  - highlighting matched terms
- Keep bundle size reasonable:
  - lazy-load index if necessary

### Rendering
- Content should be authored as Markdown (or MDX-like) with frontmatter.
- Build step converts content to pages.
- Syntax highlighting for code blocks.

### Versions
- Docs must support multiple iced versions:
  - content stored under `content/<version>/...`
  - version switcher changes base path
- Provide “latest” alias.

### Link checking & quality gates
CI must fail if:
- Internal links are broken
- Pages missing required frontmatter fields
- Search index generation fails
- Build warnings exceed a threshold (prefer “deny warnings” where feasible)

---

## Repo structure (suggested)
- `/app`               Dioxus site code
- `/content`
  - `/latest`
    - `/guide`
    - `/reference`
    - `/tutorial`
  - `/<version>`       older versions
- `/examples`          runnable rust examples referenced by docs
- `/scripts`
  - build content index
  - validate frontmatter
  - link checker
- `/public`            static assets (images, icons)
- `/docs-meta`         build/version metadata templates

---

## Page templates (enforce consistency)
Implement 3 page templates:
1) GuidePage
2) ReferencePage
3) TutorialPage

Each template:
- Reads frontmatter
- Renders standardized sections
- Shows metadata (version, last updated)
- Auto-generates TOC

ReferencePage must enforce section headers:
- Overview
- API
- Examples
- Pitfalls
- Related

---

## Styling / design requirements
- “Nice looking” means:
  - clean typography, good spacing
  - light + dark mode
  - responsive layout
  - consistent component styling
  - callout blocks: Note / Tip / Warning
- Use a cohesive design system:
  - tokens for spacing/typography/colors
  - no random one-off CSS values

---

## Example policy (runnable examples)
- Every example should live in `/examples/<name>`
- Each example should have:
  - README snippet or brief description
  - build instructions
- Docs pages reference examples by path and (optionally) embed code excerpts.
- Avoid duplicating large code blocks directly in Markdown unless small.

---

## Documentation accuracy policy
- Treat iced API as versioned.
- If you are unsure about an API detail:
  - mark TODO with a consistent tag: `TODO(api-verify)`
  - do not invent signatures
- Prefer linking to iced official docs/source where possible.

---

## Deliverables checklist
- [ ] Dioxus app renders all content pages
- [ ] Sidebar + version switcher + search
- [ ] Guide section complete (minimum pages listed)
- [ ] Reference section seeded with key widgets + core concepts
- [ ] Tutorial section with 3–4 tutorials
- [ ] Examples compile and are referenced correctly
- [ ] CI checks: build + validate + link check
- [ ] CONTRIBUTING.md: how to add a page/widget, how to add a new version

---

## Agent execution plan (step-by-step)
1) Scaffold Dioxus site + routing + layouts
2) Implement content loader (Markdown + frontmatter) + TOC generator
3) Implement sidebar tree from content directory structure
4) Implement search indexing + UI
5) Implement versioning paths + switcher
6) Build templates (Guide/Reference/Tutorial)
7) Seed Guide pages and minimal Reference pages
8) Add `/examples` and wire “view source”/excerpts
9) Add validation scripts + link checking + CI workflow
10) Polish styling + a11y (keyboard nav, contrast, focus states)

---

## Acceptance criteria (must pass)
- `cargo build` and `cargo test` pass for site + examples (where applicable)
- `npm`/tooling is only used if truly necessary; prefer Rust-native pipeline
- No broken internal links
- Search returns correct pages for widget names
- Switching versions changes content without breaking nav
- Lighthouse-like basics: fast load, minimal JS, accessible nav (best-effort)

---

## “Don’t get stuck” heuristics
- If a decision is subjective, choose the simplest consistent option and document it.
- Prefer stable, boring dependencies.
- When unknown, add TODO(api-verify) rather than hallucinating.

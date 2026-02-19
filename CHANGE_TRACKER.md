# Change Tracker

## 2026-02-19

### Summary

Completed a full refactor from a single hardcoded Dioxus page into a content-driven iced documentation site aligned to `SITE_SPEC.md` requirements, using `ref/doc` and `ref/examples` as the verification source.

### Implemented

- Replaced monolithic `src/main.rs` app with modular architecture:
- `src/main.rs` now launches `app::app`.
- New `src/app/mod.rs` contains routing, content ingestion, sidebar generation, version switching, search, markdown rendering, TOC, heading deep links, and prev/next nav.

- Added build-time content index generation:
- New `build.rs` recursively scans `src/content/**/*.md`.
- Generates `OUT_DIR/content_gen.rs` with embedded markdown (`include_str!`) so site runtime does not depend on `ref/`.

- Added versioned markdown content under `src/content/latest`:
- Guide pages:
  - `src/content/latest/guide/overview.md`
  - `src/content/latest/guide/installation.md`
  - `src/content/latest/guide/project-setup.md`
  - `src/content/latest/guide/tooling.md`
  - `src/content/latest/guide/bundling.md`
  - `src/content/latest/guide/distribution.md`
- Reference pages:
  - `src/content/latest/reference/core-concepts.md`
  - `src/content/latest/reference/runtime-api.md`
  - `src/content/latest/reference/widgets-overview.md`
  - `src/content/latest/reference/tasks-subscriptions.md`
- Tutorial pages:
  - `src/content/latest/tutorial/basic-window-button.md`
  - `src/content/latest/tutorial/layout-input.md`
  - `src/content/latest/tutorial/async-tasks.md`
  - `src/content/latest/tutorial/theming-components.md`

- Added frontmatter requirements and validation support:
- Required metadata fields enforced in tests:
  - `title`
  - `description`
  - `version`
  - `last_updated`
- Added `order` support for deterministic nav ordering.

- Implemented required site features from spec:
- Sidebar generated from `/src/content` structure.
- Version selector dropdown (works with current/future version folders).
- On-page TOC from `h2`/`h3` headings.
- Deep-linkable headings with heading anchors.
- Client-side search index over title/description/section/body.
- Copy buttons on code blocks (clipboard write via `document::eval`).
- Prev/next page navigation.

- Reworked styling in `assets/main.css` for docs layout:
- Top bar with version/search actions.
- Sticky sidebar and TOC.
- Responsive behavior for tablet/mobile.
- Styled markdown blocks and code/copy affordances.

- Added quality-gate automation script:
- `scripts/ci_quality_gates.sh` runs:
  - `cargo fmt --check`
  - `RUSTFLAGS="-D warnings" cargo check`
  - frontmatter/link/search validation tests.

### Verification Against `ref/`

Used `ref/doc/iced` and `ref/examples` to ground docs content and API claims:

- Rustdoc checks:
  - `ref/doc/iced/fn.run.html`
  - `ref/doc/iced/fn.application.html`
  - `ref/doc/iced/fn.exit.html`
  - `ref/doc/iced/struct.Task.html`
  - `ref/doc/iced/struct.Subscription.html`
  - `ref/doc/iced/widget/fn.button.html`
  - `ref/doc/iced/widget/fn.text_input.html`

- Example checks:
  - `ref/examples/counter/src/main.rs`
  - `ref/examples/tour/src/main.rs`
  - `ref/examples/todos/src/main.rs`
  - `ref/examples/todos/README.md`

### Build/Test Results

Executed successfully:

- `cargo fmt`
- `cargo check`
- `cargo test`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

### Notes

- `ref/` remained read-only and unchanged.
- Content rendering source remains strictly `/src/content`.
- No Dioxus pre-0.7 APIs were introduced.

## 2026-02-19 (Reference Expansion Pass 2)

### Summary

Expanded the Reference section to comprehensive per-item coverage for widget modules, constructors, and element structs from iced rustdoc. Added dedicated runtime-function pages with deeper guidance for how/when/why each function is used.

### Implemented

- Added generator script for reference coverage:
- `scripts/generate_reference_pages.sh`
- Source of truth for generation:
  - `ref/doc/iced/widget/sidebar-items.js`
  - `ref/doc/iced/sidebar-items.js`

- Generated per-item Reference pages under `src/content/latest/reference`:
- Widget modules: 28 pages (`widget-module-*.md`)
- Widget constructors/helpers: 44 pages (`widget-constructor-*.md`)
- Widget element structs: 30 pages (`widget-element-*.md`)

- Added catalog index pages:
- `src/content/latest/reference/widget-modules-catalog.md`
- `src/content/latest/reference/widget-constructors-catalog.md`
- `src/content/latest/reference/widget-elements-catalog.md`

- Added/enriched dedicated runtime function pages:
- `src/content/latest/reference/runtime-fn-run.md`
- `src/content/latest/reference/runtime-fn-application.md`
- `src/content/latest/reference/runtime-fn-daemon.md`
- `src/content/latest/reference/runtime-fn-exit.md`
- `src/content/latest/reference/runtime-fn-never.md`

- Updated runtime and widget overview pages to route users into per-item docs:
- `src/content/latest/reference/runtime-api.md`
- `src/content/latest/reference/widgets-overview.md`

### Verification Against `ref/`

- Runtime top-level functions confirmed from:
  - `ref/doc/iced/index.html`
  - `ref/doc/iced/sidebar-items.js`
- Widget coverage lists confirmed from:
  - `ref/doc/iced/widget/index.html`
  - `ref/doc/iced/widget/sidebar-items.js`
- Runtime usage examples cross-checked in:
  - `ref/examples/counter/src/main.rs` (`run`)
  - `ref/examples/tour/src/main.rs` (`application`)
  - `ref/examples/multi_window/src/main.rs` (`daemon`, `exit`)
  - `ref/examples/changelog/src/main.rs` (`exit`)

### Build/Test Results

Executed successfully after expansion:

- `cargo fmt`
- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (Reference Expansion Pass 3 - Signature/Example Enrichment)

### Summary

Enhanced generated reference pages so each runtime/widget page now includes extracted rustdoc signatures (or declarations), rustdoc summary text, and example-file references sourced from `ref/examples`.

### Implemented

- Upgraded generator workflow in:
- `scripts/generate_reference_pages.sh`

- Added extraction helpers in generator:
- Signature extraction from rustdoc item declarations (`<pre class="rust item-decl">`)
- Summary extraction from `ref/doc/iced/widget/index.html` item `<dd>` blocks
- Example-file discovery via `rg` against `ref/examples/**/*.rs`

- Re-generated all generated reference pages with enriched sections:
- `## Verified signature` / `## Verified type declaration`
- `## Rustdoc summary`
- `## Example References`
- Runtime function pages now include expanded `When to use` / `Why to use` guidance plus examples.

### Verification Against `ref/`

Data extraction sources used by generation:

- `ref/doc/iced/sidebar-items.js`
- `ref/doc/iced/widget/sidebar-items.js`
- `ref/doc/iced/widget/index.html`
- `ref/doc/iced/fn.*.html`
- `ref/doc/iced/widget/fn.*.html`
- `ref/doc/iced/widget/struct.*.html`
- `ref/examples/**/*.rs`

### Build/Test Results

Executed successfully after enrichment:

- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (UI Pass - Collapsible Sidebar)

### Summary

Refactored the sidebar navigation to reduce vertical overload by introducing collapsible menus at section and subsection levels.

### Implemented

- Updated sidebar rendering in:
- `src/app/mod.rs`

- Added top-level collapsible sections (`Guide`, `Reference`, `Tutorial`) using `<details>/<summary>`.
- Added reference-specific collapsible subgroups to split the long list:
  - Runtime and Core
  - Catalogs
  - Widget Modules
  - Constructors A-M
  - Constructors N-Z
  - Elements A-M
  - Elements N-Z
  - Other (fallback)

- Auto-open behavior:
- The section containing the active page opens automatically.
- The matching reference subgroup containing the active page also opens automatically.

- Added helper functions for grouping/open-state logic:
- `section_has_active_page`
- `reference_subgroups`
- `split_half`
- `push_group`

- Updated sidebar styles in:
- `assets/main.css`

- Added visual styles for section/subsection toggle rows and item counts.
- Added subgroup separators and adjusted sidebar spacing for better scanability.

### Build/Test Results

Executed successfully after UI changes:

- `cargo fmt`
- `cargo check`
- `cargo test`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (Routing Pass - Simplified Catalog/Subpath URLs)

### Summary

Updated reference routing to use simplified catalog index URLs and nested subpaths, as requested.

### Implemented

- Extended router to support nested doc paths:
- `src/app/mod.rs`
- Added route variant:
  - `/:version/:section/:group/:slug`

- Introduced canonical route mapping for reference families:
- Catalog indexes:
  - `/latest/reference/widget-modules`
  - `/latest/reference/widget-constructors`
  - `/latest/reference/widget-elements`
- Item pages:
  - `/latest/reference/widget-modules/<name>`
  - `/latest/reference/widget-constructors/<name>`
  - `/latest/reference/widget-elements/<name>`

- Added routing helpers:
- `slug_to_route_path`
- `canonical_route_path`
- `route_from_path`
- Refactored `Doc` rendering into shared `render_doc` + `DocNested` route component.

- Added `route_lookup` index to catalog for route-based page resolution.

- Updated link validation test to support both 3-segment and 4-segment internal routes and validate against canonical route lookup.

- Updated reference content generation to emit simplified URLs:
- `scripts/generate_reference_pages.sh`
- Regenerated reference pages with new canonical links.

- Updated manual reference overview links:
- `src/content/latest/reference/widgets-overview.md`

### Build/Test Results

Executed successfully after routing changes:

- `cargo fmt`
- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (UI Pass - Catalog Header Link Behavior)

### Summary

Adjusted reference sidebar subgroup behavior so catalog pages are reached by clicking subgroup header text, while clicking elsewhere on the subgroup header row toggles collapse.

### Implemented

- Removed separate `Catalogs` subgroup from reference sidebar grouping.
- Updated reference subgroup rendering in `src/app/mod.rs`:
  - Subgroup label text is now a link to the relevant catalog route.
  - Count badge and non-link header area retain toggle behavior.
- Catalog-link mapping added:
  - `Widget Modules` -> `/{version}/reference/widget-modules`
  - `Constructors A-M` / `Constructors N-Z` -> `/{version}/reference/widget-constructors`
  - `Elements A-M` / `Elements N-Z` -> `/{version}/reference/widget-elements`
  - `Runtime and Core` -> `/{version}/reference/runtime-api`
- Added `.subgroup-link` styling in `assets/main.css`.

### Build/Test Results

Executed successfully after sidebar behavior update:

- `cargo fmt`
- `cargo check`
- `cargo test`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (Naming + Content Structure Pass)

### Summary

Updated reference naming and content organization to remove the `Widget` prefix from user-facing labels, cleaned item titles/menu labels, and migrated generated reference content into nested folder structure.

### Implemented

- Updated sidebar group labels in `src/app/mod.rs`:
  - `Widget Modules` -> `Modules`
  - Catalog submenu removed previously and retained
  - Constructors/Elements groups preserved with A-M / N-Z split

- Sidebar item label cleanup:
  - Added `sidebar_display_title` so menu entries display concise names.
  - Example: `Module - Combo Box` is shown in menu as `Combo Box`.

- Page title cleanup in generated content:
  - Module pages now use titles like `Module - Combo Box`
  - Constructor pages use `Constructor - Bottom Right`
  - Element pages use `Element - Vertical Slider`

- Updated reference route naming conventions:
  - `/latest/reference/modules`
  - `/latest/reference/modules/<name>`
  - `/latest/reference/constructors`
  - `/latest/reference/constructors/<name>`
  - `/latest/reference/elements`
  - `/latest/reference/elements/<name>`

- Updated canonical slug/path mapping logic in `src/app/mod.rs`:
  - Supports new nested slugs (`modules/<name>`, `constructors/<name>`, `elements/<name>`)
  - Keeps compatibility aliases for previous `widget-*` patterns in route mapping helpers.

- Migrated generated content structure (under `/src/content`):
  - From flat generated files like `.../widget-constructor-bottom_right.md`
  - To nested structure:
    - `src/content/latest/reference/modules/<name>.md`
    - `src/content/latest/reference/constructors/<name>.md`
    - `src/content/latest/reference/elements/<name>.md`
  - Catalog/index files now:
    - `src/content/latest/reference/modules.md`
    - `src/content/latest/reference/constructors.md`
    - `src/content/latest/reference/elements.md`

- Reworked generator:
  - `scripts/generate_reference_pages.sh`
  - Emits new nested file paths, cleaned titles, simplified links, and regenerated all reference pages.

- Updated overview links:
  - `src/content/latest/reference/widgets-overview.md`

### Build/Test Results

Executed successfully after naming and structure refactor:

- `cargo fmt`
- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (Generator Stability Pass - Persisted Example Mapping)

### Summary

Fixed `scripts/generate_reference_pages.sh` so regenerated pages no longer lose curated example links. Added a persistent manual example map and merged it with auto-discovered examples from `ref/examples`.

### Implemented

- Added persistent example mapping file:
- `src/docs-meta/example_map.tsv`

- Updated generator (`scripts/generate_reference_pages.sh`) to merge example sources:
- Auto-discovery via `rg` pattern matching in `ref/examples/**/*.rs`
- Directory inference (`ref/examples/<name>/src/main.rs`) for module/constructor/element names
- Manual curated mappings from `src/docs-meta/example_map.tsv`
- Stable de-dup + cap (`merge_examples`) to avoid noisy lists

- Result:
- Running `./scripts/generate_reference_pages.sh` now preserves curated examples from the map instead of replacing them with TODO placeholders.
- Additional examples are discovered automatically where naming patterns or folder names match.

### Verification Against `ref/`

Example links now derive from:

- `ref/examples/**/*.rs` (searched)
- Explicit map entries in `src/docs-meta/example_map.tsv`

### Build/Test Results

Executed successfully after generator stability changes:

- `cargo fmt`
- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (Example Map Expansion Across ref/examples)

### Summary

Expanded `src/docs-meta/example_map.tsv` by scanning all Rust files under `ref/examples` and mapping each example to runtime/module/constructor/element items it uses. Generation now preserves and grows example references instead of dropping curated entries.

### Implemented

- Added new map expansion script:
- `scripts/expand_example_map.sh`

- Script behavior:
- Scans all `ref/examples/**/*.rs` files.
- Detects runtime usage (`run`, `application`, `daemon`, `exit`, `never` when present).
- Detects constructors by function-call usage.
- Detects modules by explicit `widget::<module>` references plus constructor-to-module inference.
- Detects element entries by explicit type references plus constructor-to-element inference.
- Merges discovered data with existing manual map entries.
- Rewrites `src/docs-meta/example_map.tsv` deterministically (sorted, de-duplicated).

- Updated generator integration:
- `scripts/generate_reference_pages.sh` now consumes the expanded map and merges it with automatic discovery and directory inference.

- Regenerated reference content so expanded example references are reflected in pages.

### Coverage Snapshot

From expanded `src/docs-meta/example_map.tsv`:

- runtime entries: 4
- module entries: 24
- constructor entries: 38
- element entries: 27

Remaining pages with TODO example placeholder after expansion: 13.

### Build/Test Results

Executed successfully after map expansion and regeneration:

- `scripts/expand_example_map.sh`
- `scripts/generate_reference_pages.sh`
- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

Validation tests passing:

- `app::tests::all_pages_have_required_frontmatter`
- `app::tests::internal_links_resolve`
- `app::tests::search_index_builds`

## 2026-02-19 (Generator Hardening - Backtick-Safe Templates)

### Summary

Hardened `scripts/generate_reference_pages.sh` against shell command-substitution breakage caused by backticks in unquoted heredoc templates.

### Implemented

- Removed all backtick literals from generator templates.
- Replaced markdown code fences with `~~~` style in generated sections.
- Kept generated output semantics intact (headers/source/signature blocks still rendered).

### Why this fixes it

In `cat <<PAGE` heredocs, backticks can trigger command substitution. Eliminating backticks in template text prevents accidental command execution if inline references are edited.

### Verification

- `./scripts/generate_reference_pages.sh`
- `cargo check`
- `cargo test`
- `scripts/ci_quality_gates.sh`

All passed.

## 2026-02-19 (Markdown Parsing Migration - Dioxus 0.7 Compatible)

### Summary

Replaced the hand-rolled markdown block/inline parser in the site renderer with `pulldown-cmark` parsing/rendering, while preserving current UX features (TOC, deep heading links, code-copy buttons). Also documented why `dioxus-markdown = 0.0.1` could not be mounted directly in this Dioxus 0.7 app.

### Implemented

- Updated markdown rendering in:
- `src/app/mod.rs`

- Removed custom parser pipeline:
- Deleted internal markdown block/inline enums and parsing functions (`parse_blocks`, `parse_inline`).

- Added markdown rendering via parser library:
- New `render_markdown_html` uses `pulldown-cmark` (`Parser::new_ext` + `html::push_html`).
- Enabled parsing features: tables, footnotes, strikethrough, and task lists.

- Preserved existing page behavior after render:
- Kept TOC generation from h2/h3 via new `extract_toc_items`.
- Added heading-text cleanup helper (`clean_heading_text`) so TOC labels/slugs stay readable.
- Added DOM enhancement pass (`document::eval`) that:
  - assigns deterministic heading ids to h2/h3,
  - injects deep-link `#` anchors,
  - wraps code blocks with existing `.code-block` chrome and copy button.

- Dependency updates:
- `Cargo.toml`: added `pulldown-cmark = "0.13"`.

### Compatibility Note

- `dioxus-markdown = 0.0.1` currently depends on `dioxus 0.6.x`, which is type-incompatible with this repo’s required `dioxus 0.7.x` component system. Attempting to mount `dioxus_markdown::Markdown` in RSX fails with mixed `dioxus_core` versions.
- This pass therefore uses a Dioxus-0.7-safe parser/renderer path while keeping the same site functionality.

### Build/Test Results

Executed successfully after migration:

- `cargo fmt`
- `cargo check`

## 2026-02-19 (Markdown Code Block Fix - Render + Copy Restore)

### Summary

Fixed a regression where fenced code blocks were disappearing after markdown rendering, and restored the copy-button behavior on those blocks.

### Root Cause

- In `src/app/mod.rs`, the DOM enhancement logic wrapped `<pre>` nodes in a new `.code-block` container using the wrong operation order:
- `wrapper.append(head, pre)` was called before `parent.insertBefore(wrapper, pre)`.
- This moved `pre` out of its parent first, so the subsequent `insertBefore(..., pre)` reference became invalid and the block could be dropped from display.

### Implemented

- Updated code-block wrapping order in:
- `src/app/mod.rs`
- New order:
  - `parent.insertBefore(wrapper, pre)`
  - `wrapper.append(head, pre)`

- Updated markdown HTML injection in:
- `src/app/mod.rs`
- `dangerous_inner_html` now receives a precomputed rendered string value directly instead of interpolated string formatting, ensuring proper HTML insertion.

### Result

- Triple-backtick fenced code blocks render again in doc pages.
- Existing click-to-copy control is reattached to each rendered code block and writes code text to clipboard.

### Build/Test Results

Executed successfully after the fix:

- `cargo fmt`
- `cargo check`
- `cargo test`

## 2026-02-19 (Syntax Highlighting for Fenced Code Blocks)

### Summary

Added proper syntax highlighting for fenced markdown code blocks while preserving the existing code block UI and click-to-copy behavior.

### Implemented

- Dependency update:
- `Cargo.toml`
  - Added `syntect = "5.3.0"`

- Markdown rendering upgrade in:
- `src/app/mod.rs`
  - `render_markdown_html` now intercepts fenced/indented code block events from `pulldown-cmark`.
  - Non-code markdown still renders through `pulldown_cmark::html::push_html`.
  - Code blocks are highlighted via `syntect::html::highlighted_html_for_string`.
  - Added helper functions:
    - `extract_fence_language`
    - `syntect_assets`
    - `highlight_theme`
    - `highlight_code_block_html`
    - `add_pre_attributes`
    - `sanitize_lang_token`
    - `escape_html`

- DOM enhancement update:
- `src/app/mod.rs`
  - Copy-wrapper script now reads `data-lang` from `<pre>` first, then falls back to `language-*` classes.
  - This keeps language labels correct for both syntect-rendered and standard fenced blocks.

### Result

- Triple-backtick fenced blocks render with syntax-highlighted HTML.
- Existing copy button remains available and copies raw code text.
- Existing heading anchors/TOC behavior remains unchanged.

### Build/Test Results

Executed successfully after highlighting integration:

- `cargo fmt`
- `cargo check`
- `cargo test`

## 2026-02-19 (WASM Build Fix for `dx serve`)

### Summary

Fixed `wasm32-unknown-unknown` build failures introduced by syntax-highlighting dependencies, so web builds used by `dx serve` compile successfully.

### Root Cause

- `syntect` was added with default features, which enables `default-onig`.
- That pulled in `onig_sys`, which failed to compile for `wasm32-unknown-unknown` (C toolchain/stdlib expectations incompatible with this target in our setup).

### Implemented

- Updated dependency features in:
- `Cargo.toml`
  - `syntect` now uses:
    - `default-features = false`
    - `features = ["default-fancy"]`
- This keeps highlighting support while avoiding the `onig` backend.

### Verification

- `cargo build --target wasm32-unknown-unknown` now succeeds.
- `cargo check` succeeds.
- `dx serve` could not be fully smoke-tested in the sandbox because binding `127.0.0.1:8080` is disallowed (`Operation not permitted`), but the prior compile error path is resolved.

## 2026-02-19 (Load-Flash Removal for Anchors and Code Blocks)

### Summary

Removed initial UI flash on page load by moving heading-anchor and code-block container generation from runtime DOM mutation to markdown render-time HTML generation.

### Root Cause

- The previous implementation rendered plain markdown first, then used `document::eval` to:
  - inject `h2/h3` heading anchors,
  - wrap `<pre>` blocks with `.code-block/.code-head`,
  - attach copy handlers.
- That post-render mutation caused visible layout/content changes (flash) on initial paint.

### Implemented

- Updated markdown renderer in:
- `src/app/mod.rs`
  - `render_markdown_html` now emits final heading HTML for `h2/h3` with stable ids and anchor links directly at render time.
  - Code blocks are now emitted directly as:
    - `.code-block` wrapper
    - `.code-head` language label
    - `.copy-btn` with `data-copy`
    - highlighted `<pre>` content

- Added helpers in:
- `src/app/mod.rs`
  - `heading_plain_text`
  - `stable_heading_id`
  - `escape_html_attr`

- Simplified runtime JS enhancement in `render_doc`:
  - Removed all structural DOM rewrites.
  - Kept only lightweight copy-button event binding (`.copy-btn`), with idempotent `data-bound` guard.

### Result

- No visible load flash for heading anchors or code block containers.
- Deep links remain stable and match TOC ids.
- Click-to-copy remains functional.
- Syntax highlighting remains intact.

### Build/Test Results

Executed successfully after flash-removal refactor:

- `cargo fmt`
- `cargo check`
- `cargo test`
- `cargo build --target wasm32-unknown-unknown`

## 2026-02-19 (Markdown Rust Code-Block Formatter Script)

### Summary

Added a utility script to run `rustfmt` over fenced Rust code blocks inside markdown files.

### Implemented

- New script:
- `scripts/rustfmt_markdown_code_blocks.sh`

- Behavior:
- Scans markdown files (`.md`) and targets fenced blocks labeled `rust` or `rs`.
- Runs `rustfmt --emit stdout --edition 2021` on each matched block.
- Rewrites files in place with formatted block content.
- If a snippet is not formatable by `rustfmt`, the original snippet is preserved (safe fallback).

- CLI modes:
- Default: formats files in place.
- `--check`: reports files that would change and exits non-zero if changes are needed.
- Optional file-path arguments to limit scope.

### Verification

- Script smoke tests:
  - `scripts/rustfmt_markdown_code_blocks.sh --check src/content/latest/guide/overview.md`
  - `scripts/rustfmt_markdown_code_blocks.sh --check src/content/latest/**/*.md`
- The broader check mode completed and reported pending formatting candidates without mutating files.

### Follow-up Fixes

- Fixed rustfmt output pollution for certain snippets:
  - Switched formatting execution from `rustfmt <temp-file>` to stdin/stdout piping (`rustfmt --emit stdout < block`).
  - This avoids temp-path header lines like `/tmp/...:` being inserted into markdown blocks.

- Added recursive directory mode:
  - New `-r` / `--recursive` flag scans all `.md` files under provided directories.
  - Example:
    - `scripts/rustfmt_markdown_code_blocks.sh -r src/content`
    - `scripts/rustfmt_markdown_code_blocks.sh --check -r src/content`

- Improved CLI compatibility and parsing:
  - Flags are now accepted in any position (e.g. `path -r`, `-r path`, `--check path -r`).
  - Added fallback file discovery when `rg` is unavailable (uses `find`).
  - Replaced `mapfile` with portable read loops to support older bash versions on macOS.

## 2026-02-19 (Reference Signature Extraction Fix)

### Summary

Fixed incomplete/flattened `Verified signature` extraction in generated reference pages (constructors/elements/runtime), including cases like `constructors/button.md`.

### Root Cause

- `scripts/generate_reference_pages.sh` used a very lossy extraction pipeline:
  - tag splitting + generic HTML stripping + whitespace collapsing.
- Rustdoc signatures often include nested markup (notably `<div class="where">...</div>`), and the old pipeline collapsed/garbled declaration formatting.

### Implemented

- Replaced `extract_signature` logic in:
- `scripts/generate_reference_pages.sh`
  - New extractor uses a single-pass Perl capture of `<pre class="rust item-decl"><code>...</code></pre>`.
  - Preserves line structure and `where` block formatting.
  - Decodes core HTML entities (`&lt;`, `&gt;`, `&amp;`, etc.) without flattening declaration structure.

- Regenerated reference pages:
- `./scripts/generate_reference_pages.sh`

### Verification

- Spot-check confirmed `src/content/latest/reference/constructors/button.md` now contains full multi-line signature with proper `where` bounds.
- `cargo check` passes after regeneration.

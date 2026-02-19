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

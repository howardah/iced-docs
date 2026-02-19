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

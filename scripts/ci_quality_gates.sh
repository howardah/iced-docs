#!/usr/bin/env bash
set -euo pipefail

# Enforce formatting and warnings-as-errors.
cargo fmt --check
RUSTFLAGS="-D warnings" cargo check

# Content validations: frontmatter, internal links, and search index generation.
cargo test app::tests::all_pages_have_required_frontmatter
cargo test app::tests::internal_links_resolve
cargo test app::tests::search_index_builds

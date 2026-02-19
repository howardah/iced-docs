use dioxus::document;
use dioxus::prelude::*;

mod catalog;
mod generated;
mod markdown;
mod routing;
mod sidebar;
mod types;

use catalog::{capitalize, catalog};
#[cfg(test)]
use catalog::{build_catalog, parse_frontmatter};
use markdown::{extract_toc_items, render_markdown_html};
use routing::{canonical_route_path, escape_js_string, route_from_path, slug_to_route_path, slugify};
use sidebar::SidebarNav;
use types::DocPage;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const SECTION_ORDER: [&str; 3] = ["guide", "reference", "tutorial"];

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SiteLayout)]
    #[route("/")]
    Home {},
    #[route("/search")]
    Search {},
    #[route("/:version/:section/:slug")]
    Doc {
        version: String,
        section: String,
        slug: String,
    },
    #[route("/:version/:section/:group/:slug")]
    DocNested {
        version: String,
        section: String,
        group: String,
        slug: String,
    },
}

pub fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn SiteLayout() -> Element {
    let current_route = use_route::<Route>();
    let nav = navigator();
    let catalog = catalog();
    let current_page = current_doc_page(&current_route);
    let selected_version = current_page
        .as_ref()
        .map(|page| page.version.clone())
        .or_else(|| catalog.versions.first().cloned())
        .unwrap_or_else(|| "latest".to_string());

    let mut query = use_signal(String::new);
    let query_lower = query().to_lowercase();
    let search_results: Vec<_> = if query_lower.is_empty() {
        Vec::new()
    } else {
        catalog
            .search
            .iter()
            .filter(|entry| entry.haystack.contains(&query_lower))
            .take(8)
            .collect()
    };

    rsx! {
        div { class: "page-bg" }
        div { class: "shell",
            header { class: "topbar",
                div { class: "brand",
                    div { class: "brand-mark", "iced" }
                    div { class: "brand-copy",
                        h1 { "Iced Docs" }
                        p { "Source-verified docs generated from /src/content." }
                    }
                }
                div { class: "top-actions",
                    select {
                        class: "version-select",
                        value: "{selected_version}",
                        onchange: move |event| {
                            let target_version = event.value();
                            let destination = if let Some(page) = current_page.clone() {
                                if has_page(&target_version, &page.section, &page.slug) {
                                    route_from_path(&slug_to_route_path(
                                        &target_version,
                                        &page.section,
                                        &page.slug,
                                    ))
                                } else {
                                    first_page_route_for_version(&target_version)
                                }
                            } else {
                                first_page_route_for_version(&target_version)
                            };
                            nav.push(destination);
                        },
                        for version in &catalog.versions {
                            option { value: "{version}", "{version}" }
                        }
                    }
                    Link { class: "route-link", to: Route::Search {}, "Search" }
                    a { class: "route-link", href: "https://github.com/iced-rs/iced", target: "_blank", rel: "noreferrer", "GitHub" }
                }
            }

            div { class: "doc-shell",
                aside { class: "sidebar",
                    input {
                        class: "search-input",
                        r#type: "search",
                        value: "{query}",
                        placeholder: "Search docs",
                        oninput: move |event| query.set(event.value()),
                    }
                    if !search_results.is_empty() {
                        ul { class: "search-results",
                            for result in search_results {
                                li {
                                    a {
                                        href: "{result.route_path}",
                                        onclick: move |_| query.set(String::new()),
                                        strong { "{result.title}" }
                                        span { "{result.section}" }
                                    }
                                }
                            }
                        }
                    }
                    SidebarNav { selected_version: selected_version.clone(), current_page: current_page.clone() }
                }
                main { class: "content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let route = first_page_route_for_version("latest");
    rsx! {
        match route {
            Route::Doc { version, section, slug } => rsx! {
                Doc { version, section, slug }
            },
            Route::DocNested {
                version,
                section,
                group,
                slug,
            } => rsx! {
                DocNested { version, section, group, slug }
            },
            _ => rsx! { p { "No documentation pages found." } },
        }
    }
}

#[component]
fn Search() -> Element {
    let mut query = use_signal(String::new);
    let catalog = catalog();
    let query_lower = query().to_lowercase();

    let results: Vec<_> = if query_lower.is_empty() {
        Vec::new()
    } else {
        catalog
            .search
            .iter()
            .filter(|entry| entry.haystack.contains(&query_lower))
            .take(50)
            .collect()
    };

    rsx! {
        article { class: "doc-page",
            header { class: "doc-header",
                h1 { "Search" }
                p { "Client-side full-text search across page title, section, and markdown body." }
            }

            section { class: "card",
                input {
                    class: "search-input search-large",
                    r#type: "search",
                    value: "{query}",
                    placeholder: "Search guides, references, and tutorials",
                    oninput: move |event| query.set(event.value()),
                }
                if query().is_empty() {
                    p { class: "muted", "Enter a query to search indexed content." }
                } else if results.is_empty() {
                    p { class: "muted", "No results." }
                } else {
                    ul { class: "search-page-results",
                        for entry in results {
                            li {
                                a { href: "{entry.route_path}", "{entry.title}" }
                                span { "{entry.section}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Doc(version: String, section: String, slug: String) -> Element {
    let path = canonical_route_path(&version, &section, None, &slug);
    render_doc(path)
}

#[component]
fn DocNested(version: String, section: String, group: String, slug: String) -> Element {
    let path = canonical_route_path(&version, &section, Some(&group), &slug);
    render_doc(path)
}

fn render_doc(path: String) -> Element {
    let catalog = catalog();
    let Some(&index) = catalog.route_lookup.get(&path) else {
        return rsx! {
            article { class: "doc-page",
                header { class: "doc-header",
                    h1 { "Page not found" }
                    p { "No page for {path}" }
                }
            }
        };
    };

    let page = catalog.pages[index].clone();
    let toc_items = extract_toc_items(&page.body);
    let markdown_mount_id = format!("md-{}", slugify(&path));
    use_effect({
        let markdown_mount_id = markdown_mount_id.clone();
        move || {
            let script = format!(
                r##"
(() => {{
  const mount = document.getElementById("{mount_id}");
  if (!mount) return;

  for (const button of mount.querySelectorAll(".copy-btn")) {{
    if (button.dataset.bound === "1") continue;
    button.dataset.bound = "1";
    button.addEventListener("click", () => {{
      const text = button.getAttribute("data-copy") || "";
      if (!navigator.clipboard || !navigator.clipboard.writeText) return;
      navigator.clipboard.writeText(text);
    }});
  }}
}})();
"##,
                mount_id = escape_js_string(&markdown_mount_id),
            );
            document::eval(&script);
        }
    });

    let version_indices = catalog
        .version_lookup
        .get(&page.version)
        .cloned()
        .unwrap_or_default();
    let current_pos = version_indices
        .iter()
        .position(|candidate| *candidate == index)
        .unwrap_or(0);
    let prev_page = current_pos
        .checked_sub(1)
        .and_then(|pos| version_indices.get(pos))
        .and_then(|doc_index| catalog.pages.get(*doc_index))
        .cloned();
    let next_page = version_indices
        .get(current_pos + 1)
        .and_then(|doc_index| catalog.pages.get(*doc_index))
        .cloned();

    let rendered_markdown = render_markdown_html(&page.body);

    rsx! {
        article { class: "doc-page",
            header { class: "doc-header",
                p { class: "eyebrow", "{capitalize(&page.section)}" }
                h1 { "{page.frontmatter.title}" }
                p { class: "description", "{page.frontmatter.description}" }
                p { class: "meta", "Version: {page.frontmatter.version} | Last updated: {page.frontmatter.last_updated}" }
            }

            div { class: "doc-body-shell",
                section { class: "markdown card",
                    div { id: "{markdown_mount_id}",
                        dangerous_inner_html: rendered_markdown
                    }
                }
                aside { class: "toc card",
                    h2 { "On This Page" }
                    if toc_items.is_empty() {
                        p { class: "muted", "No headings" }
                    } else {
                        ul {
                            for (title, id) in toc_items {
                                li { a { href: "#{id}", "{title}" } }
                            }
                        }
                    }
                }
            }

            nav { class: "pager card",
                if let Some(previous) = prev_page {
                    a {
                        href: "{previous.route_path}",
                        "Previous: {previous.frontmatter.title}"
                    }
                }
                if let Some(next) = next_page {
                    a {
                        href: "{next.route_path}",
                        "Next: {next.frontmatter.title}"
                    }
                }
            }
        }
    }
}

fn current_doc_page(route: &Route) -> Option<DocPage> {
    match route {
        Route::Doc {
            version,
            section,
            slug,
        } => {
            let path = canonical_route_path(version, section, None, slug);
            catalog()
                .route_lookup
                .get(&path)
                .and_then(|index| catalog().pages.get(*index))
                .cloned()
        }
        Route::DocNested {
            version,
            section,
            group,
            slug,
        } => {
            let path = canonical_route_path(version, section, Some(group), slug);
            catalog()
                .route_lookup
                .get(&path)
                .and_then(|index| catalog().pages.get(*index))
                .cloned()
        }
        Route::Home {} => catalog().pages.first().cloned(),
        Route::Search {} => catalog().pages.first().cloned(),
    }
}

fn has_page(version: &str, section: &str, slug: &str) -> bool {
    catalog().page_lookup.contains_key(&(
        version.to_string(),
        section.to_string(),
        slug.to_string(),
    ))
}

fn first_page_route_for_version(version: &str) -> Route {
    let catalog = catalog();
    if let Some(indices) = catalog.version_lookup.get(version) {
        if let Some(first) = indices.first().and_then(|index| catalog.pages.get(*index)) {
            return route_from_path(&first.route_path);
        }
    }

    if let Some(first) = catalog.pages.first() {
        return route_from_path(&first.route_path);
    }

    Route::Home {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_pages_have_required_frontmatter() {
        for source in generated::CONTENT_SOURCES {
            let frontmatter = parse_frontmatter(source.content)
                .unwrap_or_else(|error| panic!("{}: {}", source.path, error));
            assert!(
                !frontmatter.title.trim().is_empty(),
                "missing title: {}",
                source.path
            );
            assert!(
                !frontmatter.description.trim().is_empty(),
                "missing description: {}",
                source.path
            );
            assert!(
                !frontmatter.version.trim().is_empty(),
                "missing version: {}",
                source.path
            );
            assert!(
                !frontmatter.last_updated.trim().is_empty(),
                "missing last_updated: {}",
                source.path
            );
        }
    }

    #[test]
    fn internal_links_resolve() {
        let docs = build_catalog();

        for page in &docs.pages {
            let body = &page.body;
            for link in extract_markdown_links(body) {
                if !link.starts_with('/') {
                    continue;
                }

                let segments: Vec<&str> = link.trim_matches('/').split('/').collect();
                if segments.len() != 3 && segments.len() != 4 {
                    panic!(
                        "{} contains malformed internal link {}",
                        page.route_path, link
                    );
                }

                assert!(
                    docs.route_lookup.contains_key(&link),
                    "{} links to missing page {}",
                    page.route_path,
                    link
                );
            }
        }
    }

    #[test]
    fn search_index_builds() {
        let docs = build_catalog();
        assert!(!docs.search.is_empty(), "search index is empty");
        assert_eq!(
            docs.search.len(),
            docs.pages.len(),
            "search index does not cover every page"
        );
    }

    fn extract_markdown_links(markdown: &str) -> Vec<String> {
        let mut links = Vec::new();
        let chars: Vec<char> = markdown.chars().collect();
        let mut cursor = 0;

        while cursor < chars.len() {
            if chars[cursor] == '[' {
                if let Some(close_text) = chars[cursor..].iter().position(|ch| *ch == ']') {
                    let open_href = cursor + close_text + 1;
                    if open_href < chars.len() && chars[open_href] == '(' {
                        if let Some(close_href) = chars[open_href + 1..].iter().position(|ch| *ch == ')') {
                            let href = chars[open_href + 1..open_href + 1 + close_href]
                                .iter()
                                .collect::<String>();
                            links.push(href);
                            cursor = open_href + close_href + 2;
                            continue;
                        }
                    }
                }
            }

            cursor += 1;
        }

        links
    }
}

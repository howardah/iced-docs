use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use dioxus::document;
use dioxus::prelude::*;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/content_gen.rs"));
}

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

#[derive(Debug, Clone, PartialEq)]
struct Frontmatter {
    title: String,
    description: String,
    version: String,
    last_updated: String,
    order: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct DocPage {
    version: String,
    section: String,
    slug: String,
    route_path: String,
    frontmatter: Frontmatter,
    body: String,
}

#[derive(Debug)]
struct SearchEntry {
    route_path: String,
    title: String,
    section: String,
    haystack: String,
}

#[derive(Debug)]
struct DocsCatalog {
    pages: Vec<DocPage>,
    versions: Vec<String>,
    page_lookup: HashMap<(String, String, String), usize>,
    route_lookup: HashMap<String, usize>,
    section_lookup: HashMap<(String, String), Vec<usize>>,
    version_lookup: HashMap<String, Vec<usize>>,
    search: Vec<SearchEntry>,
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
    let search_results: Vec<&SearchEntry> = if query_lower.is_empty() {
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

    let results: Vec<&SearchEntry> = if query_lower.is_empty() {
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

#[component]
fn SidebarNav(selected_version: String, current_page: Option<DocPage>) -> Element {
    let catalog = catalog();
    let current_path = current_page
        .as_ref()
        .map(|page| page.route_path.clone())
        .unwrap_or_default();

    let mut sections = Vec::new();
    for section in SECTION_ORDER {
        if let Some(indices) = catalog
            .section_lookup
            .get(&(selected_version.clone(), section.to_string()))
        {
            sections.push((section.to_string(), indices.clone()));
        }
    }

    let nav_sections: Vec<(
        String,
        Vec<usize>,
        bool,
        Vec<(String, Vec<usize>, bool, Option<String>)>,
    )> = sections
        .into_iter()
        .map(|(section, indices)| {
            let is_open = section_has_active_page(&indices, &current_path, &catalog.pages);
            let subgroups = if section == "reference" {
                reference_subgroups(&indices, &catalog.pages)
                    .into_iter()
                    .map(|(label, group_indices)| {
                        let is_group_open =
                            section_has_active_page(&group_indices, &current_path, &catalog.pages);
                        let catalog_link = subgroup_catalog_path(&selected_version, &label);
                        (label, group_indices, is_group_open, catalog_link)
                    })
                    .collect()
            } else {
                Vec::new()
            };
            (section, indices, is_open, subgroups)
        })
        .collect();

    rsx! {
        nav { class: "sidebar-nav",
            for (section, indices, is_open, subgroups) in nav_sections {
                details {
                    class: "sidebar-section",
                    open: is_open,
                    summary {
                        span { class: "section-title", "{capitalize(&section)}" }
                        span { class: "section-count", "{indices.len()}" }
                    }
                    section { class: "sidebar-group",
                        if section == "reference" {
                            for (label, group_indices, is_group_open, catalog_link) in subgroups {
                                details {
                                    class: "sidebar-subgroup",
                                    open: is_group_open,
                                    summary {
                                        if let Some(link) = catalog_link {
                                            a {
                                                class: "subgroup-link",
                                                href: "{link}",
                                                onclick: move |event| event.stop_propagation(),
                                                "{label}"
                                            }
                                        } else {
                                            span { "{label}" }
                                        }
                                        span { class: "section-count", "{group_indices.len()}" }
                                    }
                                    ul {
                                        for index in group_indices {
                                            if let Some(page) = catalog.pages.get(index) {
                                                li {
                                                    a {
                                                        class: if page.route_path == current_path { "active" } else { "" },
                                                        href: "{page.route_path}",
                                                        "{sidebar_display_title(page)}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            ul {
                                for index in indices {
                                    if let Some(page) = catalog.pages.get(index) {
                                        li {
                                            a {
                                                class: if page.route_path == current_path { "active" } else { "" },
                                                href: "{page.route_path}",
                                                "{sidebar_display_title(page)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
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

fn catalog() -> &'static DocsCatalog {
    static CATALOG: OnceLock<DocsCatalog> = OnceLock::new();
    CATALOG.get_or_init(build_catalog)
}

fn build_catalog() -> DocsCatalog {
    let mut pages: Vec<DocPage> = generated::CONTENT_SOURCES
        .iter()
        .filter_map(|entry| {
            let relative = entry.path.trim_matches('/');
            let pieces: Vec<&str> = relative.split('/').collect();
            if pieces.len() < 3 {
                return None;
            }

            let version = pieces[0].to_string();
            let section = pieces[1].to_string();
            let mut slug_parts = pieces[2..].to_vec();
            let last = slug_parts.pop()?;
            slug_parts.push(last.trim_end_matches(".md"));
            let slug = slug_parts.join("/");
            let frontmatter = parse_frontmatter(entry.content).ok()?;

            let route_path = slug_to_route_path(&version, &section, &slug);
            Some(DocPage {
                version,
                section,
                slug,
                route_path,
                frontmatter,
                body: strip_frontmatter(entry.content),
            })
        })
        .collect();

    pages.sort_by(|a, b| {
        let version_cmp = a.version.cmp(&b.version);
        if version_cmp != std::cmp::Ordering::Equal {
            return version_cmp;
        }

        let section_cmp = section_rank(&a.section).cmp(&section_rank(&b.section));
        if section_cmp != std::cmp::Ordering::Equal {
            return section_cmp;
        }

        let order_cmp = a.frontmatter.order.cmp(&b.frontmatter.order);
        if order_cmp != std::cmp::Ordering::Equal {
            return order_cmp;
        }

        a.frontmatter.title.cmp(&b.frontmatter.title)
    });

    let mut versions = BTreeSet::new();
    let mut page_lookup = HashMap::new();
    let mut route_lookup = HashMap::new();
    let mut section_lookup: HashMap<(String, String), Vec<usize>> = HashMap::new();
    let mut version_lookup: HashMap<String, Vec<usize>> = HashMap::new();
    let mut search = Vec::new();

    for (index, page) in pages.iter().enumerate() {
        versions.insert(page.version.clone());
        page_lookup.insert(
            (
                page.version.clone(),
                page.section.clone(),
                page.slug.clone(),
            ),
            index,
        );
        route_lookup.insert(page.route_path.clone(), index);
        section_lookup
            .entry((page.version.clone(), page.section.clone()))
            .or_default()
            .push(index);
        version_lookup
            .entry(page.version.clone())
            .or_default()
            .push(index);
        search.push(SearchEntry {
            route_path: page.route_path.clone(),
            title: page.frontmatter.title.clone(),
            section: page.section.clone(),
            haystack: format!(
                "{} {} {} {}",
                page.frontmatter.title,
                page.frontmatter.description,
                page.section,
                plain_text(&page.body)
            )
            .to_lowercase(),
        });
    }

    let mut versions: Vec<String> = versions.into_iter().collect();
    versions.sort_by(|a, b| match (a.as_str(), b.as_str()) {
        ("latest", "latest") => std::cmp::Ordering::Equal,
        ("latest", _) => std::cmp::Ordering::Less,
        (_, "latest") => std::cmp::Ordering::Greater,
        _ => b.cmp(a),
    });

    DocsCatalog {
        pages,
        versions,
        page_lookup,
        route_lookup,
        section_lookup,
        version_lookup,
        search,
    }
}

fn parse_frontmatter(raw: &str) -> Result<Frontmatter, String> {
    if !raw.starts_with("---\n") {
        return Err("missing frontmatter start".to_string());
    }

    let Some(frontmatter_end) = raw[4..].find("\n---\n") else {
        return Err("missing frontmatter end".to_string());
    };

    let frontmatter = &raw[4..frontmatter_end + 4];
    let mut title = None;
    let mut description = None;
    let mut version = None;
    let mut last_updated = None;
    let mut order = 999_u32;

    for line in frontmatter.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Some((key, value)) = trimmed.split_once(':') else {
            continue;
        };
        let value = value.trim().to_string();

        match key.trim() {
            "title" => title = Some(value),
            "description" => description = Some(value),
            "version" => version = Some(value),
            "last_updated" => last_updated = Some(value),
            "order" => {
                order = value.parse::<u32>().map_err(|_| "invalid order")?;
            }
            _ => {}
        }
    }

    Ok(Frontmatter {
        title: title.ok_or_else(|| "missing title".to_string())?,
        description: description.ok_or_else(|| "missing description".to_string())?,
        version: version.ok_or_else(|| "missing version".to_string())?,
        last_updated: last_updated.ok_or_else(|| "missing last_updated".to_string())?,
        order,
    })
}

fn strip_frontmatter(raw: &str) -> String {
    if !raw.starts_with("---\n") {
        return raw.to_string();
    }

    let Some(frontmatter_end) = raw[4..].find("\n---\n") else {
        return raw.to_string();
    };

    raw[frontmatter_end + 9..].to_string()
}

fn extract_toc_items(markdown: &str) -> Vec<(String, String)> {
    let mut items = Vec::new();
    let mut seen = HashMap::<String, usize>::new();
    let mut in_fence = false;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        let heading = trimmed
            .strip_prefix("### ")
            .or_else(|| trimmed.strip_prefix("## "));
        let Some(raw) = heading else {
            continue;
        };

        let title = clean_heading_text(raw);
        if title.is_empty() {
            continue;
        }

        let base = slugify(&title);
        let count = seen.entry(base.clone()).or_insert(0);
        let id = if *count == 0 {
            base
        } else {
            format!("{}-{}", base, *count)
        };
        *count += 1;
        items.push((title, id));
    }

    items
}

fn clean_heading_text(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let mut out = String::new();
    let mut cursor = 0;

    while cursor < chars.len() {
        if chars[cursor] == '`' {
            if let Some(end) = chars[cursor + 1..].iter().position(|ch| *ch == '`') {
                out.push_str(
                    &chars[cursor + 1..cursor + 1 + end]
                        .iter()
                        .collect::<String>(),
                );
                cursor += end + 2;
                continue;
            }
        }

        if chars[cursor] == '[' {
            if let Some(close_text) = chars[cursor..].iter().position(|ch| *ch == ']') {
                let open_href = cursor + close_text + 1;
                if open_href < chars.len() && chars[open_href] == '(' {
                    if let Some(close_href) =
                        chars[open_href + 1..].iter().position(|ch| *ch == ')')
                    {
                        out.push_str(
                            &chars[cursor + 1..cursor + close_text]
                                .iter()
                                .collect::<String>(),
                        );
                        cursor = open_href + close_href + 2;
                        continue;
                    }
                }
            }
        }

        if !matches!(chars[cursor], '*' | '_' | '~') {
            out.push(chars[cursor]);
        }
        cursor += 1;
    }

    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn render_markdown_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(markdown, options);
    let mut html_out = String::new();
    let mut passthrough_events: Vec<Event<'_>> = Vec::new();
    let mut events = parser.into_iter();
    let mut heading_seen = HashMap::<String, usize>::new();

    while let Some(event) = events.next() {
        match event {
            Event::Start(Tag::Heading { level, .. })
                if level == pulldown_cmark::HeadingLevel::H2
                    || level == pulldown_cmark::HeadingLevel::H3 =>
            {
                if !passthrough_events.is_empty() {
                    html::push_html(&mut html_out, passthrough_events.drain(..));
                }

                let mut heading_events: Vec<Event<'_>> = Vec::new();
                for heading_event in events.by_ref() {
                    if matches!(heading_event, Event::End(TagEnd::Heading(_))) {
                        break;
                    }
                    heading_events.push(heading_event);
                }

                let heading_text = heading_plain_text(&heading_events);
                let heading_id = stable_heading_id(&heading_text, &mut heading_seen);
                let mut heading_inner = String::new();
                html::push_html(&mut heading_inner, heading_events.into_iter());

                let heading_tag = if level == pulldown_cmark::HeadingLevel::H2 {
                    "h2"
                } else {
                    "h3"
                };
                html_out.push_str(&format!(
                    "<{tag} id=\"{id}\"><a href=\"#{id}\" class=\"heading-link\">#</a> {inner}</{tag}>",
                    tag = heading_tag,
                    id = escape_html_attr(&heading_id),
                    inner = heading_inner,
                ));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                if !passthrough_events.is_empty() {
                    html::push_html(&mut html_out, passthrough_events.drain(..));
                }

                let language = match kind {
                    CodeBlockKind::Fenced(info) => extract_fence_language(info.as_ref()),
                    CodeBlockKind::Indented => "text".to_string(),
                };

                let mut raw_code = String::new();
                for code_event in events.by_ref() {
                    match code_event {
                        Event::End(TagEnd::CodeBlock) => break,
                        Event::Text(text) | Event::Code(text) => raw_code.push_str(text.as_ref()),
                        Event::SoftBreak | Event::HardBreak => raw_code.push('\n'),
                        _ => {}
                    }
                }

                html_out.push_str(&highlight_code_block_html(&raw_code, &language));
            }
            _ => passthrough_events.push(event),
        }
    }

    if !passthrough_events.is_empty() {
        html::push_html(&mut html_out, passthrough_events.drain(..));
    }

    html_out
}

fn extract_fence_language(info: &str) -> String {
    let first = info.split_whitespace().next().unwrap_or("text");
    let language = first.split(',').next().unwrap_or("text").trim();
    if language.is_empty() {
        "text".to_string()
    } else {
        language.to_string()
    }
}

fn syntect_assets() -> &'static (SyntaxSet, ThemeSet) {
    static ASSETS: OnceLock<(SyntaxSet, ThemeSet)> = OnceLock::new();
    ASSETS.get_or_init(|| {
        (
            SyntaxSet::load_defaults_newlines(),
            ThemeSet::load_defaults(),
        )
    })
}

fn highlight_theme() -> &'static Theme {
    let (_, themes) = syntect_assets();
    themes
        .themes
        .get("base16-ocean.dark")
        .or_else(|| themes.themes.values().next())
        .expect("syntect theme set should include at least one theme")
}

fn highlight_code_block_html(code: &str, language: &str) -> String {
    let language = sanitize_lang_token(language);
    let (syntax_set, _) = syntect_assets();
    let syntax = syntax_set
        .find_syntax_by_token(&language)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let pre_html = match highlighted_html_for_string(code, syntax_set, syntax, highlight_theme()) {
        Ok(rendered) => add_pre_attributes(rendered, &language),
        Err(_) => format!(
            "<pre data-lang=\"{}\"><code>{}</code></pre>",
            escape_html_attr(&language),
            escape_html(code)
        ),
    };

    format!(
        "<div class=\"code-block\"><div class=\"code-head\"><span class=\"code-lang\">{lang}</span><button class=\"copy-btn\" type=\"button\" data-copy=\"{copy}\">Copy</button></div>{pre}</div>",
        lang = escape_html(&language),
        copy = escape_html_attr(code),
        pre = pre_html,
    )
}

fn add_pre_attributes(mut html_snippet: String, language: &str) -> String {
    if let Some(start) = html_snippet.find("<pre") {
        if let Some(end) = html_snippet[start..].find('>') {
            let insert_at = start + end;
            html_snippet.insert_str(
                insert_at,
                &format!(
                    " data-lang=\"{}\" class=\"syntect\"",
                    escape_html_attr(language)
                ),
            );
        }
    }
    html_snippet
}

fn sanitize_lang_token(value: &str) -> String {
    let normalized: String = value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '+'))
        .collect();
    if normalized.is_empty() {
        "text".to_string()
    } else {
        normalized.to_lowercase()
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_html_attr(value: &str) -> String {
    escape_html(value)
        .replace('"', "&quot;")
        .replace('\n', "&#10;")
}

fn heading_plain_text(events: &[Event<'_>]) -> String {
    let mut out = String::new();
    for event in events {
        match event {
            Event::Text(text) | Event::Code(text) => out.push_str(text.as_ref()),
            Event::SoftBreak | Event::HardBreak => out.push(' '),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn stable_heading_id(title: &str, seen: &mut HashMap<String, usize>) -> String {
    let base = {
        let slug = slugify(title);
        if slug.is_empty() {
            "section".to_string()
        } else {
            slug
        }
    };
    let count = seen.entry(base.clone()).or_insert(0);
    let id = if *count == 0 {
        base
    } else {
        format!("{}-{}", base, *count)
    };
    *count += 1;
    id
}

fn section_rank(section: &str) -> usize {
    SECTION_ORDER
        .iter()
        .position(|known| known == &section)
        .unwrap_or(usize::MAX)
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

fn section_has_active_page(indices: &[usize], current_path: &str, pages: &[DocPage]) -> bool {
    indices.iter().any(|index| {
        pages
            .get(*index)
            .is_some_and(|page| page.route_path == current_path)
    })
}

fn reference_subgroups(indices: &[usize], pages: &[DocPage]) -> Vec<(String, Vec<usize>)> {
    let mut core_runtime = Vec::new();
    let mut modules = Vec::new();
    let mut constructors_a_m = Vec::new();
    let mut constructors_n_z = Vec::new();
    let mut elements_a_m = Vec::new();
    let mut elements_n_z = Vec::new();
    let mut other = Vec::new();

    for index in indices {
        let Some(page) = pages.get(*index) else {
            continue;
        };
        let slug = page.slug.as_str();

        if matches!(slug, "modules" | "constructors" | "elements") {
            continue;
        }

        if matches!(
            slug,
            "core-concepts" | "runtime-api" | "tasks-subscriptions" | "widgets-overview"
        ) || slug.starts_with("runtime-fn-")
        {
            core_runtime.push(*index);
        } else if slug.starts_with("modules/") {
            modules.push(*index);
        } else if slug.starts_with("constructors/") {
            if split_half(&page.frontmatter.title) {
                constructors_n_z.push(*index);
            } else {
                constructors_a_m.push(*index);
            }
        } else if slug.starts_with("elements/") {
            if split_half(&page.frontmatter.title) {
                elements_n_z.push(*index);
            } else {
                elements_a_m.push(*index);
            }
        } else {
            other.push(*index);
        }
    }

    let mut groups = Vec::new();
    push_group(&mut groups, "Runtime and Core", core_runtime);
    push_group(&mut groups, "Modules", modules);
    push_group(&mut groups, "Constructors A-M", constructors_a_m);
    push_group(&mut groups, "Constructors N-Z", constructors_n_z);
    push_group(&mut groups, "Elements A-M", elements_a_m);
    push_group(&mut groups, "Elements N-Z", elements_n_z);
    push_group(&mut groups, "Other", other);
    groups
}

fn split_half(title: &str) -> bool {
    let candidate = title
        .split_once(" - ")
        .map(|(_, value)| value)
        .unwrap_or(title);
    let ch = candidate
        .chars()
        .find(|ch| ch.is_ascii_alphabetic())
        .unwrap_or('A')
        .to_ascii_uppercase();
    ch >= 'N'
}

fn push_group(groups: &mut Vec<(String, Vec<usize>)>, label: &str, indices: Vec<usize>) {
    if indices.is_empty() {
        return;
    }
    groups.push((label.to_string(), indices));
}

fn sidebar_display_title(page: &DocPage) -> String {
    for prefix in ["Module - ", "Constructor - ", "Element - "] {
        if let Some(rest) = page.frontmatter.title.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    page.frontmatter.title.clone()
}

fn subgroup_catalog_path(version: &str, label: &str) -> Option<String> {
    match label {
        "Modules" => Some(format!("/{}/reference/modules", version)),
        "Constructors A-M" | "Constructors N-Z" => {
            Some(format!("/{}/reference/constructors", version))
        }
        "Elements A-M" | "Elements N-Z" => Some(format!("/{}/reference/elements", version)),
        "Runtime and Core" => Some(format!("/{}/reference/runtime-api", version)),
        _ => None,
    }
}

fn plain_text(markdown: &str) -> String {
    markdown
        .replace('#', " ")
        .replace('`', " ")
        .replace('[', " ")
        .replace(']', " ")
        .replace('(', " ")
        .replace(')', " ")
}

fn slugify(input: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }

    output.trim_matches('-').to_string()
}

fn canonical_route_path(version: &str, section: &str, group: Option<&str>, slug: &str) -> String {
    if section != "reference" {
        return format!("/{}/{}/{}", version, section, slug);
    }

    match group {
        Some("modules" | "widget-modules") => {
            format!("/{}/{}/modules/{}", version, section, slug)
        }
        Some("constructors" | "widget-constructors") => {
            format!("/{}/{}/constructors/{}", version, section, slug)
        }
        Some("elements" | "widget-elements") => {
            format!("/{}/{}/elements/{}", version, section, slug)
        }
        _ => match slug {
            "modules" | "widget-modules" => format!("/{}/{}/modules", version, section),
            "constructors" | "widget-constructors" => {
                format!("/{}/{}/constructors", version, section)
            }
            "elements" | "widget-elements" => format!("/{}/{}/elements", version, section),
            _ => format!("/{}/{}/{}", version, section, slug),
        },
    }
}

fn slug_to_route_path(version: &str, section: &str, slug: &str) -> String {
    if section != "reference" {
        return format!("/{}/{}/{}", version, section, slug);
    }

    if slug == "modules" || slug == "widget-modules-catalog" {
        format!("/{}/{}/modules", version, section)
    } else if let Some(tail) = slug.strip_prefix("modules/") {
        format!("/{}/{}/modules/{}", version, section, tail)
    } else if let Some(tail) = slug.strip_prefix("widget-module-") {
        format!("/{}/{}/modules/{}", version, section, tail)
    } else if slug == "constructors" || slug == "widget-constructors-catalog" {
        format!("/{}/{}/constructors", version, section)
    } else if let Some(tail) = slug.strip_prefix("constructors/") {
        format!("/{}/{}/constructors/{}", version, section, tail)
    } else if let Some(tail) = slug.strip_prefix("widget-constructor-") {
        format!("/{}/{}/constructors/{}", version, section, tail)
    } else if slug == "elements" || slug == "widget-elements-catalog" {
        format!("/{}/{}/elements", version, section)
    } else if let Some(tail) = slug.strip_prefix("elements/") {
        format!("/{}/{}/elements/{}", version, section, tail)
    } else if let Some(tail) = slug.strip_prefix("widget-element-") {
        format!("/{}/{}/elements/{}", version, section, tail)
    } else {
        format!("/{}/{}/{}", version, section, slug)
    }
}

fn route_from_path(path: &str) -> Route {
    let segments: Vec<&str> = path.trim_matches('/').split('/').collect();
    match segments.as_slice() {
        [version, section, slug] => Route::Doc {
            version: (*version).to_string(),
            section: (*section).to_string(),
            slug: (*slug).to_string(),
        },
        [version, section, group, slug] => Route::DocNested {
            version: (*version).to_string(),
            section: (*section).to_string(),
            group: (*group).to_string(),
            slug: (*slug).to_string(),
        },
        _ => Route::Home {},
    }
}

fn escape_js_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
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
                        if let Some(close_href) =
                            chars[open_href + 1..].iter().position(|ch| *ch == ')')
                        {
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

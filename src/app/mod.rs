use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use dioxus::document;
use dioxus::prelude::*;

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

#[derive(Debug, Clone)]
enum Block {
    Heading { level: u8, text: String, id: String },
    Paragraph(String),
    List(Vec<String>),
    Code { lang: String, code: String },
}

#[derive(Debug, Clone)]
enum Inline {
    Text(String),
    Code(String),
    Link { text: String, href: String },
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
    let blocks = parse_blocks(&page.body);
    let toc_items: Vec<(String, String)> = blocks
        .iter()
        .filter_map(|block| match block {
            Block::Heading { level, text, id } if *level == 2 || *level == 3 => {
                Some((text.clone(), id.clone()))
            }
            _ => None,
        })
        .collect();

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
                    for block in blocks {
                        match block {
                            Block::Heading { level: 2, text, id } => rsx! {
                                h2 { id: "{id}", a { href: "#{id}", class: "heading-link", "#" } " {text}" }
                            },
                            Block::Heading { level: 3, text, id } => rsx! {
                                h3 { id: "{id}", a { href: "#{id}", class: "heading-link", "#" } " {text}" }
                            },
                            Block::Heading { .. } => rsx! { },
                            Block::Paragraph(text) => rsx! {
                                p {
                                    for span in parse_inline(&text) {
                                        match span {
                                            Inline::Text(value) => rsx! { span { "{value}" } },
                                            Inline::Code(value) => rsx! { code { "{value}" } },
                                            Inline::Link { text, href } => rsx! { a { href: "{href}", "{text}" } },
                                        }
                                    }
                                }
                            },
                            Block::List(items) => rsx! {
                                ul {
                                    for item in items {
                                        li {
                                            for span in parse_inline(&item) {
                                                match span {
                                                    Inline::Text(value) => rsx! { span { "{value}" } },
                                                    Inline::Code(value) => rsx! { code { "{value}" } },
                                                    Inline::Link { text, href } => rsx! { a { href: "{href}", "{text}" } },
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Block::Code { lang, code } => {
                                let copy_text = code.clone();
                                rsx! {
                                    div { class: "code-block",
                                        div { class: "code-head",
                                            span { class: "code-lang", "{lang}" }
                                            button {
                                                class: "copy-btn",
                                                onclick: move |_| {
                                                    let script = format!(
                                                        "navigator.clipboard && navigator.clipboard.writeText(\"{}\")",
                                                        escape_js_string(&copy_text),
                                                    );
                                                    document::eval(&script);
                                                },
                                                "Copy"
                                            }
                                        }
                                        pre { code { "{code}" } }
                                    }
                                }
                            }
                        }
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
                                                        "{page.frontmatter.title}"
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
                                                "{page.frontmatter.title}"
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
            if pieces.len() != 3 {
                return None;
            }

            let version = pieces[0].to_string();
            let section = pieces[1].to_string();
            let slug = pieces[2].trim_end_matches(".md").to_string();
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

fn parse_blocks(markdown: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut lines = markdown.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim_end();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(lang) = trimmed.strip_prefix("```") {
            let mut code = String::new();
            for code_line in lines.by_ref() {
                if code_line.trim_start().starts_with("```") {
                    break;
                }
                code.push_str(code_line);
                code.push('\n');
            }
            blocks.push(Block::Code {
                lang: lang.trim().to_string(),
                code: code.trim_end().to_string(),
            });
            continue;
        }

        if let Some(text) = trimmed.strip_prefix("## ") {
            blocks.push(Block::Heading {
                level: 2,
                text: text.trim().to_string(),
                id: slugify(text),
            });
            continue;
        }

        if let Some(text) = trimmed.strip_prefix("### ") {
            blocks.push(Block::Heading {
                level: 3,
                text: text.trim().to_string(),
                id: slugify(text),
            });
            continue;
        }

        if trimmed.starts_with('#') {
            continue;
        }

        if let Some(item) = trimmed.strip_prefix("- ") {
            let mut items = vec![item.to_string()];
            while let Some(next_line) = lines.peek() {
                if let Some(next_item) = next_line.trim_start().strip_prefix("- ") {
                    items.push(next_item.to_string());
                    lines.next();
                } else {
                    break;
                }
            }
            blocks.push(Block::List(items));
            continue;
        }

        let mut paragraph = trimmed.to_string();
        while let Some(next_line) = lines.peek() {
            let next_trimmed = next_line.trim();
            if next_trimmed.is_empty()
                || next_trimmed.starts_with("```")
                || next_trimmed.starts_with("## ")
                || next_trimmed.starts_with("### ")
                || next_trimmed.starts_with("- ")
            {
                break;
            }
            paragraph.push(' ');
            paragraph.push_str(next_trimmed);
            lines.next();
        }
        blocks.push(Block::Paragraph(paragraph));
    }

    blocks
}

fn parse_inline(text: &str) -> Vec<Inline> {
    let mut out = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut cursor = 0;

    while cursor < chars.len() {
        if chars[cursor] == '`' {
            if let Some(end) = chars[cursor + 1..].iter().position(|ch| *ch == '`') {
                let code = chars[cursor + 1..cursor + 1 + end]
                    .iter()
                    .collect::<String>();
                out.push(Inline::Code(code));
                cursor += end + 2;
                continue;
            }
        }

        if chars[cursor] == '[' {
            let rest = &chars[cursor..];
            if let Some(close_text) = rest.iter().position(|ch| *ch == ']') {
                let next = cursor + close_text + 1;
                if next + 1 < chars.len() && chars[next] == '(' {
                    if let Some(close_href) = chars[next + 1..].iter().position(|ch| *ch == ')') {
                        let text_value = chars[cursor + 1..cursor + close_text]
                            .iter()
                            .collect::<String>();
                        let href_value = chars[next + 1..next + 1 + close_href]
                            .iter()
                            .collect::<String>();
                        out.push(Inline::Link {
                            text: text_value,
                            href: href_value,
                        });
                        cursor = next + close_href + 2;
                        continue;
                    }
                }
            }
        }

        let mut next_special = cursor + 1;
        while next_special < chars.len() && chars[next_special] != '`' && chars[next_special] != '['
        {
            next_special += 1;
        }
        out.push(Inline::Text(chars[cursor..next_special].iter().collect()));
        cursor = next_special;
    }

    out
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

        if slug.ends_with("-catalog") || slug.contains("catalog") {
            continue;
        }

        if matches!(
            slug,
            "core-concepts" | "runtime-api" | "tasks-subscriptions" | "widgets-overview"
        ) || slug.starts_with("runtime-fn-")
        {
            core_runtime.push(*index);
        } else if slug.starts_with("widget-module-") {
            modules.push(*index);
        } else if slug.starts_with("widget-constructor-") {
            if split_half(&page.frontmatter.title) {
                constructors_n_z.push(*index);
            } else {
                constructors_a_m.push(*index);
            }
        } else if slug.starts_with("widget-element-") {
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
    push_group(&mut groups, "Widget Modules", modules);
    push_group(&mut groups, "Constructors A-M", constructors_a_m);
    push_group(&mut groups, "Constructors N-Z", constructors_n_z);
    push_group(&mut groups, "Elements A-M", elements_a_m);
    push_group(&mut groups, "Elements N-Z", elements_n_z);
    push_group(&mut groups, "Other", other);
    groups
}

fn split_half(title: &str) -> bool {
    let ch = title
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

fn subgroup_catalog_path(version: &str, label: &str) -> Option<String> {
    match label {
        "Widget Modules" => Some(format!("/{}/reference/widget-modules", version)),
        "Constructors A-M" | "Constructors N-Z" => {
            Some(format!("/{}/reference/widget-constructors", version))
        }
        "Elements A-M" | "Elements N-Z" => Some(format!("/{}/reference/widget-elements", version)),
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
        Some("widget-modules") => format!("/{}/{}/widget-modules/{}", version, section, slug),
        Some("widget-constructors") => {
            format!("/{}/{}/widget-constructors/{}", version, section, slug)
        }
        Some("widget-elements") => format!("/{}/{}/widget-elements/{}", version, section, slug),
        _ => match slug {
            "widget-modules" => format!("/{}/{}/widget-modules", version, section),
            "widget-constructors" => format!("/{}/{}/widget-constructors", version, section),
            "widget-elements" => format!("/{}/{}/widget-elements", version, section),
            _ => format!("/{}/{}/{}", version, section, slug),
        },
    }
}

fn slug_to_route_path(version: &str, section: &str, slug: &str) -> String {
    if section != "reference" {
        return format!("/{}/{}/{}", version, section, slug);
    }

    if slug == "widget-modules-catalog" {
        format!("/{}/{}/widget-modules", version, section)
    } else if let Some(tail) = slug.strip_prefix("widget-module-") {
        format!("/{}/{}/widget-modules/{}", version, section, tail)
    } else if slug == "widget-constructors-catalog" {
        format!("/{}/{}/widget-constructors", version, section)
    } else if let Some(tail) = slug.strip_prefix("widget-constructor-") {
        format!("/{}/{}/widget-constructors/{}", version, section, tail)
    } else if slug == "widget-elements-catalog" {
        format!("/{}/{}/widget-elements", version, section)
    } else if let Some(tail) = slug.strip_prefix("widget-element-") {
        format!("/{}/{}/widget-elements/{}", version, section, tail)
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

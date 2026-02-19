use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use super::generated;
use super::routing::slug_to_route_path;
use super::types::{DocPage, DocsCatalog, Frontmatter, SearchEntry};
use super::SECTION_ORDER;

pub(crate) fn catalog() -> &'static DocsCatalog {
    static CATALOG: OnceLock<DocsCatalog> = OnceLock::new();
    CATALOG.get_or_init(build_catalog)
}

pub(crate) fn build_catalog() -> DocsCatalog {
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

pub(crate) fn parse_frontmatter(raw: &str) -> Result<Frontmatter, String> {
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

fn section_rank(section: &str) -> usize {
    SECTION_ORDER
        .iter()
        .position(|known| known == &section)
        .unwrap_or(usize::MAX)
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

pub(crate) fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

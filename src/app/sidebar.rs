use std::collections::HashMap;

use dioxus::prelude::*;

use super::catalog::{capitalize, catalog};
use super::types::DocPage;
use super::SECTION_ORDER;

#[component]
pub(crate) fn SidebarNav(selected_version: String, current_page: Option<DocPage>) -> Element {
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
                                    if label == "Widgets" {
                                        for (family_title, family_path, module_path, constructor_path, element_path, family_open, item_count) in
                                            widget_family_menu_items(&group_indices, &catalog.pages, &current_path)
                                        {
                                            details {
                                                class: "sidebar-subgroup widget-family",
                                                open: family_open,
                                                summary {
                                                    if let Some(path) = family_path.clone() {
                                                        a {
                                                            class: "subgroup-link",
                                                            href: "{path}",
                                                            onclick: move |event| event.stop_propagation(),
                                                            "{family_title}"
                                                        }
                                                    } else {
                                                        span { "{family_title}" }
                                                    }
                                                    span { class: "section-count", "{item_count}" }
                                                }
                                                ul {
                                                    if let Some(path) = module_path {
                                                        li {
                                                            a {
                                                                class: if path == current_path { "active" } else { "" },
                                                                href: "{path}",
                                                                "Module"
                                                            }
                                                        }
                                                    }
                                                    if let Some(path) = constructor_path {
                                                        li {
                                                            a {
                                                                class: if path == current_path { "active" } else { "" },
                                                                href: "{path}",
                                                                "Constructor"
                                                            }
                                                        }
                                                    }
                                                    if let Some(path) = element_path {
                                                        li {
                                                            a {
                                                                class: if path == current_path { "active" } else { "" },
                                                                href: "{path}",
                                                                "Element"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
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

fn section_has_active_page(indices: &[usize], current_path: &str, pages: &[DocPage]) -> bool {
    indices.iter().any(|index| {
        pages
            .get(*index)
            .is_some_and(|page| page.route_path == current_path)
    })
}

fn reference_subgroups(indices: &[usize], pages: &[DocPage]) -> Vec<(String, Vec<usize>)> {
    let mut core_runtime = Vec::new();
    let mut enums = Vec::new();
    let mut structs = Vec::new();
    let mut widgets = Vec::new();
    let mut other = Vec::new();

    for index in indices {
        let Some(page) = pages.get(*index) else {
            continue;
        };
        let slug = page.slug.as_str();

        if matches!(
            slug,
            "families" | "modules" | "constructors" | "elements" | "enums" | "structs"
        ) {
            continue;
        }

        if matches!(
            slug,
            "core-concepts" | "runtime-api" | "tasks-subscriptions" | "widgets-overview"
        ) || slug.starts_with("runtime-fn-")
        {
            core_runtime.push(*index);
        } else if slug.starts_with("enums/") {
            enums.push(*index);
        } else if slug.starts_with("structs/") {
            structs.push(*index);
        } else if slug == "families"
            || slug.starts_with("families/")
            || slug == "modules"
            || slug.starts_with("modules/")
            || slug == "constructors"
            || slug.starts_with("constructors/")
            || slug == "elements"
            || slug.starts_with("elements/")
        {
            widgets.push(*index);
        } else {
            other.push(*index);
        }
    }

    let mut groups = Vec::new();
    push_group(&mut groups, "Runtime and Core", core_runtime);
    push_group(&mut groups, "Enums", enums);
    push_group(&mut groups, "Structs", structs);
    push_group(&mut groups, "Widgets", widgets);
    push_group(&mut groups, "Other", other);
    groups
}

fn push_group(groups: &mut Vec<(String, Vec<usize>)>, label: &str, indices: Vec<usize>) {
    if indices.is_empty() {
        return;
    }
    groups.push((label.to_string(), indices));
}

fn sidebar_display_title(page: &DocPage) -> String {
    for prefix in [
        "Family - ",
        "Module - ",
        "Constructor - ",
        "Element - ",
        "Struct - ",
        "Enum - ",
    ] {
        if let Some(rest) = page.frontmatter.title.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    page.frontmatter.title.clone()
}

fn subgroup_catalog_path(version: &str, label: &str) -> Option<String> {
    match label {
        "Enums" => Some(format!("/{}/reference/enums", version)),
        "Structs" => Some(format!("/{}/reference/structs", version)),
        "Widgets" => Some(format!("/{}/reference/families", version)),
        "Runtime and Core" => Some(format!("/{}/reference/runtime-api", version)),
        _ => None,
    }
}

fn widget_family_slug(page: &DocPage) -> Option<String> {
    let slug = page.slug.as_str();
    if slug == "families" || slug == "modules" || slug == "constructors" || slug == "elements" {
        return None;
    }
    if let Some(value) = slug.strip_prefix("families/") {
        return Some(value.to_string());
    }
    if let Some(value) = slug.strip_prefix("modules/") {
        return Some(value.replace('_', "-"));
    }
    if let Some(value) = slug.strip_prefix("constructors/") {
        return Some(value.replace('_', "-"));
    }
    if let Some(value) = slug.strip_prefix("elements/") {
        return Some(value.to_string());
    }
    None
}

fn humanize_slug(slug: &str) -> String {
    slug.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn widget_family_menu_items(
    indices: &[usize],
    pages: &[DocPage],
    current_path: &str,
) -> Vec<(
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    bool,
    usize,
)> {
    let mut families: HashMap<String, (Option<usize>, Option<usize>, Option<usize>, Option<usize>)> =
        HashMap::new();

    for index in indices {
        let Some(page) = pages.get(*index) else {
            continue;
        };
        let Some(family_slug) = widget_family_slug(page) else {
            continue;
        };
        let entry = families
            .entry(family_slug)
            .or_insert((None, None, None, None));
        if page.slug.starts_with("families/") {
            entry.0 = Some(*index);
        } else if page.slug.starts_with("modules/") {
            entry.1 = Some(*index);
        } else if page.slug.starts_with("constructors/") {
            entry.2 = Some(*index);
        } else if page.slug.starts_with("elements/") {
            entry.3 = Some(*index);
        }
    }

    let mut out = Vec::new();
    for (family_slug, (family_idx, module_idx, constructor_idx, element_idx)) in families {
        let title = family_idx
            .and_then(|index| pages.get(index))
            .and_then(|page| page.frontmatter.title.strip_prefix("Family - "))
            .map(ToString::to_string)
            .unwrap_or_else(|| humanize_slug(&family_slug));

        let family_path = family_idx
            .and_then(|index| pages.get(index))
            .map(|page| page.route_path.clone());
        let module_path = module_idx
            .and_then(|index| pages.get(index))
            .map(|page| page.route_path.clone());
        let constructor_path = constructor_idx
            .and_then(|index| pages.get(index))
            .map(|page| page.route_path.clone());
        let element_path = element_idx
            .and_then(|index| pages.get(index))
            .map(|page| page.route_path.clone());

        let item_count = [
            family_path.as_ref(),
            module_path.as_ref(),
            constructor_path.as_ref(),
            element_path.as_ref(),
        ]
        .iter()
        .filter(|path| path.is_some())
        .count();
        let is_open = [
            family_path.as_ref(),
            module_path.as_ref(),
            constructor_path.as_ref(),
            element_path.as_ref(),
        ]
        .iter()
        .filter_map(|path| path.as_ref())
        .any(|path| path.as_str() == current_path);

        out.push((
            title,
            family_path,
            module_path,
            constructor_path,
            element_path,
            is_open,
            item_count,
        ));
    }

    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

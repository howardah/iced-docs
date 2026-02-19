use super::Route;

pub(crate) fn slugify(input: &str) -> String {
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

pub(crate) fn canonical_route_path(
    version: &str,
    section: &str,
    group: Option<&str>,
    slug: &str,
) -> String {
    if section != "reference" {
        return format!("/{}/{}/{}", version, section, slug);
    }

    match group {
        Some("enums") => format!("/{}/{}/enums/{}", version, section, slug),
        Some("structs") => format!("/{}/{}/structs/{}", version, section, slug),
        Some("families" | "widget-families") => {
            format!("/{}/{}/families/{}", version, section, slug)
        }
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
            "enums" => format!("/{}/{}/enums", version, section),
            "structs" => format!("/{}/{}/structs", version, section),
            "families" | "widget-families" => format!("/{}/{}/families", version, section),
            "modules" | "widget-modules" => format!("/{}/{}/modules", version, section),
            "constructors" | "widget-constructors" => {
                format!("/{}/{}/constructors", version, section)
            }
            "elements" | "widget-elements" => format!("/{}/{}/elements", version, section),
            _ => format!("/{}/{}/{}", version, section, slug),
        },
    }
}

pub(crate) fn slug_to_route_path(version: &str, section: &str, slug: &str) -> String {
    if section != "reference" {
        return format!("/{}/{}/{}", version, section, slug);
    }

    if slug == "enums" {
        format!("/{}/{}/enums", version, section)
    } else if let Some(tail) = slug.strip_prefix("enums/") {
        format!("/{}/{}/enums/{}", version, section, tail)
    } else if slug == "structs" {
        format!("/{}/{}/structs", version, section)
    } else if let Some(tail) = slug.strip_prefix("structs/") {
        format!("/{}/{}/structs/{}", version, section, tail)
    } else if slug == "modules" || slug == "widget-modules-catalog" {
        format!("/{}/{}/modules", version, section)
    } else if slug == "families" || slug == "widget-families-catalog" {
        format!("/{}/{}/families", version, section)
    } else if let Some(tail) = slug.strip_prefix("families/") {
        format!("/{}/{}/families/{}", version, section, tail)
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

pub(crate) fn route_from_path(path: &str) -> Route {
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

pub(crate) fn escape_js_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

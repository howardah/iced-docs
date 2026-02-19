use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Frontmatter {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) version: String,
    pub(crate) last_updated: String,
    pub(crate) order: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DocPage {
    pub(crate) version: String,
    pub(crate) section: String,
    pub(crate) slug: String,
    pub(crate) route_path: String,
    pub(crate) frontmatter: Frontmatter,
    pub(crate) body: String,
}

#[derive(Debug)]
pub(crate) struct SearchEntry {
    pub(crate) route_path: String,
    pub(crate) title: String,
    pub(crate) section: String,
    pub(crate) haystack: String,
}

#[derive(Debug)]
pub(crate) struct DocsCatalog {
    pub(crate) pages: Vec<DocPage>,
    pub(crate) versions: Vec<String>,
    pub(crate) page_lookup: HashMap<(String, String, String), usize>,
    pub(crate) route_lookup: HashMap<String, usize>,
    pub(crate) section_lookup: HashMap<(String, String), Vec<usize>>,
    pub(crate) version_lookup: HashMap<String, Vec<usize>>,
    pub(crate) search: Vec<SearchEntry>,
}

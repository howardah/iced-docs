use std::collections::HashMap;
use std::sync::OnceLock;

use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use super::routing::slugify;

pub(crate) fn extract_toc_items(markdown: &str) -> Vec<(String, String)> {
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
                    if let Some(close_href) = chars[open_href + 1..].iter().position(|ch| *ch == ')')
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

pub(crate) fn render_markdown_html(markdown: &str) -> String {
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

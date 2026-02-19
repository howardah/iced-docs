#!/usr/bin/env bash
set -euo pipefail

ROOT="src/content/latest/reference"
DATE="2026-02-19"

mkdir -p "$ROOT/modules" "$ROOT/constructors" "$ROOT/elements"

widget_sidebar="ref/doc/iced/widget/sidebar-items.js"
iced_sidebar="ref/doc/iced/sidebar-items.js"
widget_index="ref/doc/iced/widget/index.html"

extract_array() {
    local file="$1"
    local key="$2"
    sed -E "s/.*\"${key}\":\[([^]]+)\].*/\1/" "$file" | tr -d '"' | tr ',' '\n' | sed '/^$/d'
}

kebab_from_pascal() {
    local value="$1"
    echo "$value" | sed -E 's/([a-z0-9])([A-Z])/\1-\2/g' | tr '[:upper:]' '[:lower:]'
}

titleize_name() {
    local value="$1"
    echo "$value" | sed 's/_/ /g; s/-/ /g' | awk '{for(i=1;i<=NF;i++){ $i=toupper(substr($i,1,1)) tolower(substr($i,2)); } print}'
}

clean_html_text() {
    sed -E 's/<[^>]+>//g' | sed 's/&lt;/</g; s/&gt;/>/g; s/&amp;/\&/g; s/\&nbsp;/ /g' | sed -E 's/[[:space:]]+/ /g' | sed 's/^ //; s/ $//'
}

extract_signature() {
    local file="$1"
    sed 's/></>\n</g' "$file" \
        | sed -n '/<pre class="rust item-decl">/,/<\/pre>/p' \
        | clean_html_text \
        | sed '/^$/d'
}

extract_index_description() {
    local kind="$1"
    local name="$2"
    local title=""

    case "$kind" in
        mod) title="title=\"mod iced::widget::${name}\"" ;;
        fn) title="title=\"fn iced::widget::${name}\"" ;;
        struct) title="title=\"struct iced::widget::${name}\"" ;;
        *) echo ""; return 0 ;;
    esac

    sed 's/></>\n</g' "$widget_index" \
        | awk -v pat="$title" '
            $0 ~ pat { found=1; next }
            found && /<dd>/ { in_dd=1 }
            in_dd { print }
            in_dd && /<\/dd>/ { exit }
        ' \
        | clean_html_text
}

list_examples() {
    local pattern="$1"
    rg -l "$pattern" ref/examples -g '*.rs' 2>/dev/null | sed -n '1,6p' || true
}

render_example_section() {
    local header="$1"
    shift
    local files=("$@")

    echo "## ${header}"
    if [[ ${#files[@]} -eq 0 ]] || [[ -z "${files[0]:-}" ]]; then
        echo
        echo "- TODO(api-verify): add canonical example mapping for this item."
        return
    fi

    echo
    for file in "${files[@]}"; do
        [[ -z "$file" ]] && continue
        echo "- ${file}"
    done
}

rm -f "$ROOT"/runtime-fn-*.md
rm -f "$ROOT"/widget-*.md
rm -f "$ROOT"/modules/*.md "$ROOT"/constructors/*.md "$ROOT"/elements/*.md

runtime_order=20
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue

    runtime_order=$((runtime_order + 1))
    sig="$(extract_signature "ref/doc/iced/fn.${fn_name}.html")"
    ex_files=()

    case "$fn_name" in
        run)
            ex_files=( $(list_examples 'iced::run\(') )
            when='Use it for straightforward apps where State: Default is acceptable and you want minimal startup wiring.'
            why='It is the shortest path from update/view logic to a running app.'
            ;;
        application)
            ex_files=( $(list_examples 'iced::application\(') )
            when='Use it when you need runtime builder configuration (title/theme/window/subscription/font/presets) before run().' 
            why='It scales better for production apps with explicit startup and configuration needs.'
            ;;
        daemon)
            ex_files=( $(list_examples 'iced::daemon\(') )
            when='Use it for daemon-like or background-centric app lifecycles, including multi-window orchestration.'
            why='It provides the daemon runtime builder counterpart to application.'
            ;;
        exit)
            ex_files=( $(list_examples 'iced::exit\(') )
            when='Use it inside update logic when a message should trigger runtime shutdown.'
            why='It returns a Task so shutdown composes with the same side-effect model as other runtime actions.'
            ;;
        never)
            ex_files=( $(list_examples 'iced::never\(') )
            when='Use it only for advanced unreachable Infallible-based branches in typed/generic code.'
            why='It allows impossible branches to satisfy type requirements safely.'
            ;;
        *)
            when='See rustdoc for usage.'
            why='See rustdoc for rationale.'
            ;;
    esac

    {
        cat <<PAGE
---
title: Runtime Function - ${fn_name}
description: Detailed guidance for iced::${fn_name}.
version: latest
last_updated: ${DATE}
order: ${runtime_order}
---

# Runtime Function - iced::${fn_name}

Authoritative source: ref/doc/iced/fn.${fn_name}.html.

## Verified signature

\`\`\`rust
PAGE
        echo "$sig"
        echo '```'
        cat <<PAGE

## When to use it

${when}

## Why to use it

${why}

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
PAGE
    } > "$ROOT/runtime-fn-${fn_name}.md"
done < <(extract_array "$iced_sidebar" "fn")

module_order=100
while read -r module_name || [[ -n "$module_name" ]]; do
    [[ -z "$module_name" ]] && continue
    module_order=$((module_order + 1))

    display="$(titleize_name "$module_name")"
    description="$(extract_index_description mod "$module_name")"
    ex_files=( $(list_examples "widget::${module_name}") )

    {
        cat <<PAGE
---
title: Module - ${display}
description: Module-level reference for iced::widget::${module_name}.
version: latest
last_updated: ${DATE}
order: ${module_order}
---

# Module - ${display}

Authoritative source: ref/doc/iced/widget/${module_name}/index.html.

## Rustdoc description

${description:-TODO(api-verify)}

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::${module_name}.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## Related

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
PAGE
    } > "$ROOT/modules/${module_name}.md"
done < <(extract_array "$widget_sidebar" "mod")

constructor_order=300
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue
    constructor_order=$((constructor_order + 1))

    display="$(titleize_name "$fn_name")"
    sig="$(extract_signature "ref/doc/iced/widget/fn.${fn_name}.html")"
    description="$(extract_index_description fn "$fn_name")"
    ex_files=( $(list_examples "\\b${fn_name}\\(") )

    {
        cat <<PAGE
---
title: Constructor - ${display}
description: Function reference for iced::widget::${fn_name}.
version: latest
last_updated: ${DATE}
order: ${constructor_order}
---

# Constructor - ${display}

Authoritative source: ref/doc/iced/widget/fn.${fn_name}.html.

## Rustdoc summary

${description:-TODO(api-verify)}

## Verified signature

\`\`\`rust
PAGE
        echo "$sig"
        echo '```'
        cat <<'PAGE'

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
PAGE
    } > "$ROOT/constructors/${fn_name}.md"
done < <(extract_array "$widget_sidebar" "fn")

element_order=500
while read -r struct_name || [[ -n "$struct_name" ]]; do
    [[ -z "$struct_name" ]] && continue
    element_order=$((element_order + 1))
    slug="$(kebab_from_pascal "$struct_name")"
    display="$(titleize_name "$slug")"

    sig="$(extract_signature "ref/doc/iced/widget/struct.${struct_name}.html")"
    description="$(extract_index_description struct "$struct_name")"
    ex_files=( $(list_examples "widget::${struct_name}|\\b${struct_name}<'") )

    {
        cat <<PAGE
---
title: Element - ${display}
description: Struct reference for iced::widget::${struct_name}.
version: latest
last_updated: ${DATE}
order: ${element_order}
---

# Element - ${display}

Authoritative source: ref/doc/iced/widget/struct.${struct_name}.html.

## Rustdoc summary

${description:-TODO(api-verify)}

## Verified type declaration

\`\`\`rust
PAGE
        echo "$sig"
        echo '```'
        cat <<'PAGE'

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
PAGE
    } > "$ROOT/elements/${slug}.md"
done < <(extract_array "$widget_sidebar" "struct")

cat > "$ROOT/modules.md" <<PAGE
---
title: Modules
description: Index of all iced::widget modules exposed in rustdoc.
version: latest
last_updated: ${DATE}
order: 90
---

# Modules

Generated from ref/doc/iced/widget/sidebar-items.js.

## Module Index

PAGE
while read -r module_name || [[ -n "$module_name" ]]; do
    [[ -z "$module_name" ]] && continue
    echo "- [$(titleize_name "$module_name")](/latest/reference/modules/${module_name})" >> "$ROOT/modules.md"
done < <(extract_array "$widget_sidebar" "mod")

cat > "$ROOT/constructors.md" <<PAGE
---
title: Constructors
description: Index of all iced::widget constructor/helper functions exposed in rustdoc.
version: latest
last_updated: ${DATE}
order: 91
---

# Constructors

Generated from ref/doc/iced/widget/sidebar-items.js.

## Constructor Index

PAGE
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue
    echo "- [$(titleize_name "$fn_name")](/latest/reference/constructors/${fn_name})" >> "$ROOT/constructors.md"
done < <(extract_array "$widget_sidebar" "fn")

cat > "$ROOT/elements.md" <<PAGE
---
title: Elements
description: Index of all iced::widget element structs exposed in rustdoc.
version: latest
last_updated: ${DATE}
order: 92
---

# Elements

Generated from ref/doc/iced/widget/sidebar-items.js.

## Element Index

PAGE
while read -r struct_name || [[ -n "$struct_name" ]]; do
    [[ -z "$struct_name" ]] && continue
    slug="$(kebab_from_pascal "$struct_name")"
    echo "- [$(titleize_name "$slug")](/latest/reference/elements/${slug})" >> "$ROOT/elements.md"
done < <(extract_array "$widget_sidebar" "struct")

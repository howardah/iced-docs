#!/usr/bin/env bash
set -euo pipefail

ROOT="src/content/latest/reference"
DATE="2026-02-19"

mkdir -p "$ROOT"

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
    rg -l "$pattern" ref/examples -g '*.rs' 2>/dev/null | head -n 6 || true
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

# Clean old generated pages.
rm -f "$ROOT"/runtime-fn-*.md
rm -f "$ROOT"/widget-module-*.md
rm -f "$ROOT"/widget-constructor-*.md
rm -f "$ROOT"/widget-element-*.md
rm -f "$ROOT"/widget-modules-catalog.md "$ROOT"/widget-constructors-catalog.md "$ROOT"/widget-elements-catalog.md

# Runtime function pages.
runtime_order=20
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue

    runtime_order=$((runtime_order + 1))
    sig="$(extract_signature "ref/doc/iced/fn.${fn_name}.html")"
    ex_files=()

    case "$fn_name" in
        run)
            ex_files=( $(list_examples 'iced::run\(') )
            when='Use it for straightforward apps where `State: Default` is acceptable and you want minimal startup wiring.'
            why='It is the shortest path from update/view logic to a running app.'
            ;;
        application)
            ex_files=( $(list_examples 'iced::application\(') )
            when='Use it when you need runtime builder configuration (title/theme/window/subscription/font/presets) before `.run()`.'
            why='It scales better for production apps with explicit startup and configuration needs.'
            ;;
        daemon)
            ex_files=( $(list_examples 'iced::daemon\(') )
            when='Use it for daemon-like or background-centric app lifecycles, including multi-window orchestration.'
            why='It provides the daemon runtime builder counterpart to `application`.'
            ;;
        exit)
            ex_files=( $(list_examples 'iced::exit\(') )
            when='Use it inside update logic when a message should trigger runtime shutdown.'
            why='It returns a `Task` so shutdown composes with the same side-effect model as other runtime actions.'
            ;;
        never)
            ex_files=( $(list_examples 'iced::never\(') )
            when='Use it only for advanced unreachable `Infallible`-based branches in typed/generic code.'
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

Authoritative source: `ref/doc/iced/fn.${fn_name}.html`.

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

# Module pages.
module_order=100
while read -r module_name || [[ -n "$module_name" ]]; do
    [[ -z "$module_name" ]] && continue
    module_order=$((module_order + 1))

    description="$(extract_index_description mod "$module_name")"
    ex_files=( $(list_examples "widget::${module_name}") )

    {
        cat <<PAGE
---
title: Widget Module - ${module_name}
description: Module-level reference for iced::widget::${module_name}.
version: latest
last_updated: ${DATE}
order: ${module_order}
---

# Widget Module - iced::widget::${module_name}

Authoritative source: ref/doc/iced/widget/${module_name}/index.html.

## Rustdoc description

${description:-TODO(api-verify)}

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::${module_name}.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## Related

- [Widget Modules Catalog](/latest/reference/widget-modules-catalog)
- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
PAGE
    } > "$ROOT/widget-module-${module_name}.md"

done < <(extract_array "$widget_sidebar" "mod")

# Constructor pages.
constructor_order=300
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue
    constructor_order=$((constructor_order + 1))

    sig="$(extract_signature "ref/doc/iced/widget/fn.${fn_name}.html")"
    description="$(extract_index_description fn "$fn_name")"
    ex_files=( $(list_examples "\\b${fn_name}\\(") )

    {
        cat <<PAGE
---
title: Widget Constructor - ${fn_name}
description: Function reference for iced::widget::${fn_name}.
version: latest
last_updated: ${DATE}
order: ${constructor_order}
---

# Widget Constructor - iced::widget::${fn_name}

Authoritative source: ref/doc/iced/widget/fn.${fn_name}.html.

## Rustdoc summary

${description:-TODO(api-verify)}

## Verified signature

\`\`\`rust
PAGE
        echo "$sig"
        cat <<'PAGE'
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
PAGE
    } > "$ROOT/widget-constructor-${fn_name}.md"

done < <(extract_array "$widget_sidebar" "fn")

# Element struct pages.
element_order=500
while read -r struct_name || [[ -n "$struct_name" ]]; do
    [[ -z "$struct_name" ]] && continue
    element_order=$((element_order + 1))
    slug="$(kebab_from_pascal "$struct_name")"

    sig="$(extract_signature "ref/doc/iced/widget/struct.${struct_name}.html")"
    description="$(extract_index_description struct "$struct_name")"
    ex_files=( $(list_examples "widget::${struct_name}|\\b${struct_name}<'") )

    {
        cat <<PAGE
---
title: Widget Element - ${struct_name}
description: Struct reference for iced::widget::${struct_name}.
version: latest
last_updated: ${DATE}
order: ${element_order}
---

# Widget Element - iced::widget::${struct_name}

Authoritative source: ref/doc/iced/widget/struct.${struct_name}.html.

## Rustdoc summary

${description:-TODO(api-verify)}

## Verified type declaration

\`\`\`rust
PAGE
        echo "$sig"
        cat <<'PAGE'
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        cat <<'PAGE'

## Related

- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
PAGE
    } > "$ROOT/widget-element-${slug}.md"

done < <(extract_array "$widget_sidebar" "struct")

# Catalog pages.
cat > "$ROOT/widget-modules-catalog.md" <<PAGE
---
title: Widget Modules Catalog
description: Index of all iced::widget modules exposed in rustdoc.
version: latest
last_updated: ${DATE}
order: 90
---

# Widget Modules Catalog

Generated from ref/doc/iced/widget/sidebar-items.js.

## Modules

PAGE
while read -r module_name || [[ -n "$module_name" ]]; do
    [[ -z "$module_name" ]] && continue
    echo "- [Widget Module - ${module_name}](/latest/reference/widget-module-${module_name})" >> "$ROOT/widget-modules-catalog.md"
done < <(extract_array "$widget_sidebar" "mod")

cat > "$ROOT/widget-constructors-catalog.md" <<PAGE
---
title: Widget Constructors Catalog
description: Index of all iced::widget constructor/helper functions exposed in rustdoc.
version: latest
last_updated: ${DATE}
order: 91
---

# Widget Constructors Catalog

Generated from ref/doc/iced/widget/sidebar-items.js.

## Constructors

PAGE
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue
    echo "- [Widget Constructor - ${fn_name}](/latest/reference/widget-constructor-${fn_name})" >> "$ROOT/widget-constructors-catalog.md"
done < <(extract_array "$widget_sidebar" "fn")

cat > "$ROOT/widget-elements-catalog.md" <<PAGE
---
title: Widget Elements Catalog
description: Index of all iced::widget element structs exposed in rustdoc.
version: latest
last_updated: ${DATE}
order: 92
---

# Widget Elements Catalog

Generated from ref/doc/iced/widget/sidebar-items.js.

## Elements

PAGE
while read -r struct_name || [[ -n "$struct_name" ]]; do
    [[ -z "$struct_name" ]] && continue
    slug="$(kebab_from_pascal "$struct_name")"
    echo "- [Widget Element - ${struct_name}](/latest/reference/widget-element-${slug})" >> "$ROOT/widget-elements-catalog.md"
done < <(extract_array "$widget_sidebar" "struct")

#!/usr/bin/env bash
set -euo pipefail

ROOT="src/content/latest/reference"
DATE="2026-02-19"

mkdir -p "$ROOT/modules" "$ROOT/constructors" "$ROOT/elements"
mkdir -p "$ROOT/families"

widget_sidebar="ref/doc/iced/widget/sidebar-items.js"
iced_sidebar="ref/doc/iced/sidebar-items.js"
widget_index="ref/doc/iced/widget/index.html"
example_map_file="src/docs-meta/example_map.tsv"

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
    perl -0777 -ne '
        if (/<pre class="rust item-decl">.*?<code>(.*?)<\/code>.*?<\/pre>/s) {
            $s = $1;
            $s =~ s/<div class="where">/\n/g;
            $s =~ s/<br\s*\/?>/\n/g;
            $s =~ s/<\/p>/\n/g;
            $s =~ s/<[^>]+>//g;
            $s =~ s/&lt;/</g;
            $s =~ s/&gt;/>/g;
            $s =~ s/&amp;/\&/g;
            $s =~ s/&nbsp;/ /g;
            $s =~ s/&#39;/'\''/g;
            $s =~ s/&quot;/"/g;
            $s =~ s/\r//g;
            $s =~ s/[ \t]+\n/\n/g;
            $s =~ s/\n{3,}/\n\n/g;
            $s =~ s/^\s+//;
            $s =~ s/\s+$//;
            print "$s\n";
        }
    ' "$file"
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

list_examples_limited() {
    local pattern="$1"
    local limit="$2"
    rg -l "$pattern" ref/examples -g '*.rs' 2>/dev/null | sed -n "1,${limit}p" || true
}

infer_examples_from_dir() {
    local name="$1"
    for candidate in "$name" "${name//-/_}" "${name//_/-}"; do
        local path="ref/examples/${candidate}/src/main.rs"
        if [[ -f "$path" ]]; then
            echo "$path"
        fi
    done
}

manual_examples() {
    local kind="$1"
    local key="$2"
    if [[ ! -f "$example_map_file" ]]; then
        return
    fi
    awk -F '\t' -v k="$kind" -v id="$key" '
        $1 == k && $2 == id && $3 != "" {
            n = split($3, arr, /[ ]*,[ ]*/)
            for (i = 1; i <= n; i++) {
                if (arr[i] != "") print arr[i]
            }
        }
    ' "$example_map_file"
}

merge_examples() {
    awk '!seen[$0]++' | sed '/^$/d' | sed -n '1,6p'
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

extract_inline_examples_markdown() {
    local file="$1"
    local max_examples="${2:-3}"
    [[ -f "$file" ]] || return 0

    MAX_EXAMPLES="$max_examples" perl -0777 -ne '
        my $html = $_;
        my $max = $ENV{"MAX_EXAMPLES"} // 3;
        my $count = 0;

        sub decode_html {
            my ($s) = @_;
            $s =~ s/<br\s*\/?>/\n/g;
            $s =~ s/<\/p>/\n/g;
            $s =~ s/<[^>]+>//g;
            $s =~ s/&lt;/</g;
            $s =~ s/&gt;/>/g;
            $s =~ s/&amp;/&/g;
            $s =~ s/&nbsp;/ /g;
            $s =~ s/&#39;/'\''/g;
            $s =~ s/&quot;/"/g;
            $s =~ s/\r//g;
            $s =~ s/[ \t]+\n/\n/g;
            $s =~ s/\n{3,}/\n\n/g;
            $s =~ s/^\s+//;
            $s =~ s/\s+$//;
            return $s;
        }

        my @sections = ();
        while ($html =~ m{<h2 id="example"[^>]*>.*?</h2>(.*?)(?=<h2 id="|<h3 id="|<details class="toggle method-toggle"|</section>|\z)}sg) {
            push @sections, $1;
        }
        @sections = ($html) if scalar(@sections) == 0;

        for my $section (@sections) {
            while ($section =~ m{<pre class="[^"]*rust-example-rendered[^"]*"><code>(.*?)</code></pre>}sg) {
                my $code = decode_html($1);
                next if $code eq "";
                print "```rust\n$code\n```\n\n";
                $count++;
                last if $count >= $max;
            }
            last if $count >= $max;
        }
    ' "$file"
}

render_inline_examples_section() {
    local markdown="$1"
    if [[ -z "${markdown//[[:space:]]/}" ]]; then
        return
    fi
    cat <<PAGE
## Inline Examples (from rustdoc)

${markdown}
PAGE
}

struct_name_from_slug() {
    local slug="$1"
    while read -r struct_name || [[ -n "$struct_name" ]]; do
        [[ -z "$struct_name" ]] && continue
        if [[ "$(kebab_from_pascal "$struct_name")" == "$slug" ]]; then
            echo "$struct_name"
            return 0
        fi
    done < <(extract_array "$widget_sidebar" "struct")
    return 1
}

rm -f "$ROOT"/runtime-fn-*.md
rm -f "$ROOT"/widget-*.md
rm -f "$ROOT"/modules/*.md "$ROOT"/constructors/*.md "$ROOT"/elements/*.md
rm -f "$ROOT"/families/*.md

runtime_order=20
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue

    runtime_order=$((runtime_order + 1))
    runtime_doc="ref/doc/iced/fn.${fn_name}.html"
    sig="$(extract_signature "$runtime_doc")"
    inline_examples="$(extract_inline_examples_markdown "$runtime_doc" 2)"
    ex_files=()
    auto_files=()
    inferred_files=()
    manual_files=()

    case "$fn_name" in
        run)
            auto_files=( $(list_examples_limited 'iced::run\(' 10) )
            when='Use it for straightforward apps where State: Default is acceptable and you want minimal startup wiring.'
            why='It is the shortest path from update/view logic to a running app.'
            ;;
        application)
            auto_files=( $(list_examples_limited 'iced::application\(' 10) )
            when='Use it when you need runtime builder configuration (title/theme/window/subscription/font/presets) before run().'
            why='It scales better for production apps with explicit startup and configuration needs.'
            ;;
        daemon)
            auto_files=( $(list_examples_limited 'iced::daemon\(' 10) )
            when='Use it for daemon-like or background-centric app lifecycles, including multi-window orchestration.'
            why='It provides the daemon runtime builder counterpart to application.'
            ;;
        exit)
            auto_files=( $(list_examples_limited 'iced::exit\(' 10) )
            when='Use it inside update logic when a message should trigger runtime shutdown.'
            why='It returns a Task so shutdown composes with the same side-effect model as other runtime actions.'
            ;;
        never)
            auto_files=( $(list_examples_limited 'iced::never\(' 10) )
            when='Use it only for advanced unreachable Infallible-based branches in typed/generic code.'
            why='It allows impossible branches to satisfy type requirements safely.'
            ;;
        *)
            when='See rustdoc for usage.'
            why='See rustdoc for rationale.'
            ;;
    esac
    manual_files=( $(manual_examples runtime "$fn_name") )
    ex_files=( $(printf "%s\n" "${auto_files[@]}" "${manual_files[@]}" | merge_examples) )

    {
        cat <<PAGE
---
title: Runtime Function - ${fn_name}
description: Detailed guidance for iced::${fn_name}.
version: latest
last_updated: ${DATE}
order: ${runtime_order}
---

# Runtime Function - \`iced::${fn_name}\`

Authoritative source: \`ref/doc/iced/fn.${fn_name}.html\`.

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
        echo
        render_inline_examples_section "$inline_examples"
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
    module_doc="ref/doc/iced/widget/${module_name}/index.html"
    inline_examples="$(extract_inline_examples_markdown "$module_doc" 2)"
    ex_files=()
    auto_files=( $(list_examples_limited "widget::${module_name}" 8) )
    inferred_files=( $(infer_examples_from_dir "$module_name") )
    manual_files=( $(manual_examples module "$module_name") )
    ex_files=( $(printf "%s\n" "${auto_files[@]}" "${inferred_files[@]}" "${manual_files[@]}" | merge_examples) )

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

Authoritative source: \`ref/doc/iced/widget/${module_name}/index.html\`.

## Rustdoc description

${description:-TODO(api-verify)}

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::${module_name}.

PAGE
        render_example_section "Example References" "${ex_files[@]}"
        echo
        render_inline_examples_section "$inline_examples"
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
    constructor_doc="ref/doc/iced/widget/fn.${fn_name}.html"
    sig="$(extract_signature "$constructor_doc")"
    inline_examples="$(extract_inline_examples_markdown "$constructor_doc" 2)"
    description="$(extract_index_description fn "$fn_name")"
    ex_files=()
    auto_files=( $(list_examples_limited "\\b${fn_name}\\(" 8) )
    inferred_files=( $(infer_examples_from_dir "$fn_name") )
    manual_files=( $(manual_examples constructor "$fn_name") )
    ex_files=( $(printf "%s\n" "${auto_files[@]}" "${inferred_files[@]}" "${manual_files[@]}" | merge_examples) )

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

Authoritative source: \`ref/doc/iced/widget/fn.${fn_name}.html\`.

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
        echo
        render_inline_examples_section "$inline_examples"
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

    element_doc="ref/doc/iced/widget/struct.${struct_name}.html"
    sig="$(extract_signature "$element_doc")"
    inline_examples="$(extract_inline_examples_markdown "$element_doc" 2)"
    description="$(extract_index_description struct "$struct_name")"
    ex_files=()
    auto_files=( $(list_examples_limited "widget::${struct_name}|\\b${struct_name}<'|\\b${struct_name}\\b" 8) )
    inferred_files=( $(infer_examples_from_dir "$slug") )
    manual_files=( $(manual_examples element "$slug") )
    ex_files=( $(printf "%s\n" "${auto_files[@]}" "${inferred_files[@]}" "${manual_files[@]}" | merge_examples) )

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
        echo
        render_inline_examples_section "$inline_examples"
        cat <<'PAGE'

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
PAGE
    } > "$ROOT/elements/${slug}.md"
done < <(extract_array "$widget_sidebar" "struct")

family_order=700
family_slugs="$(
    {
        extract_array "$widget_sidebar" "mod" | sed 's/_/-/g'
        echo
        extract_array "$widget_sidebar" "fn" | sed 's/_/-/g'
        echo
        extract_array "$widget_sidebar" "struct" | while read -r struct_name || [[ -n "$struct_name" ]]; do
            [[ -z "$struct_name" ]] && continue
            kebab_from_pascal "$struct_name"
        done
    } | sed '/^$/d' | sort -u
)"

while read -r family_slug || [[ -n "$family_slug" ]]; do
    [[ -z "$family_slug" ]] && continue
    family_order=$((family_order + 1))
    display="$(titleize_name "$family_slug")"

    module_name="${family_slug//-/_}"
    constructor_name="${family_slug//-/_}"
    struct_name="$(struct_name_from_slug "$family_slug" || true)"

    has_module=0
    has_constructor=0
    has_element=0
    [[ -f "$ROOT/modules/${module_name}.md" ]] && has_module=1
    [[ -f "$ROOT/constructors/${constructor_name}.md" ]] && has_constructor=1
    [[ -f "$ROOT/elements/${family_slug}.md" ]] && has_element=1

    module_description=""
    constructor_description=""
    element_description=""
    constructor_sig=""
    element_sig=""
    constructor_inline_examples=""
    element_inline_examples=""

    if [[ $has_module -eq 1 ]]; then
        module_description="$(extract_index_description mod "$module_name")"
    fi
    if [[ $has_constructor -eq 1 ]]; then
        constructor_description="$(extract_index_description fn "$constructor_name")"
        constructor_doc="ref/doc/iced/widget/fn.${constructor_name}.html"
        constructor_sig="$(extract_signature "$constructor_doc")"
        constructor_inline_examples="$(extract_inline_examples_markdown "$constructor_doc" 1)"
    fi
    if [[ $has_element -eq 1 && -n "$struct_name" ]]; then
        element_description="$(extract_index_description struct "$struct_name")"
        element_doc="ref/doc/iced/widget/struct.${struct_name}.html"
        element_sig="$(extract_signature "$element_doc")"
        element_inline_examples="$(extract_inline_examples_markdown "$element_doc" 1)"
    fi

    ex_files=()
    auto_files=( $(list_examples_limited "widget::${module_name}|\\b${constructor_name}\\(|\\b${struct_name}\\b" 10) )
    inferred_files=( $(infer_examples_from_dir "$family_slug") )
    manual_module_files=( $(manual_examples module "$module_name") )
    manual_constructor_files=( $(manual_examples constructor "$constructor_name") )
    manual_element_files=( $(manual_examples element "$family_slug") )
    ex_files=( $(printf "%s\n" "${auto_files[@]}" "${inferred_files[@]}" "${manual_module_files[@]}" "${manual_constructor_files[@]}" "${manual_element_files[@]}" | merge_examples) )

    {
        cat <<PAGE
---
title: Family - ${display}
description: Unified reference for the ${display} widget family across module, constructor, and element APIs.
version: latest
last_updated: ${DATE}
order: ${family_order}
---

# Family - ${display}

This page unifies related iced::widget APIs for the **${display}** family.

## API surfaces

PAGE
        if [[ $has_module -eq 1 ]]; then
            echo "- Module: [iced::widget::${module_name}](/latest/reference/modules/${module_name})"
        fi
        if [[ $has_constructor -eq 1 ]]; then
            echo "- Constructor: [iced::widget::${constructor_name}](/latest/reference/constructors/${constructor_name})"
        fi
        if [[ $has_element -eq 1 ]]; then
            echo "- Element: [iced::widget::${struct_name}](/latest/reference/elements/${family_slug})"
        fi
        cat <<'PAGE'

## Surface summaries
PAGE
        if [[ $has_module -eq 1 ]]; then
            cat <<PAGE

### Module

${module_description:-TODO(api-verify)}
PAGE
        fi
        if [[ $has_constructor -eq 1 ]]; then
            cat <<PAGE

### Constructor

${constructor_description:-TODO(api-verify)}
PAGE
        fi
        if [[ $has_element -eq 1 ]]; then
            cat <<PAGE

### Element

${element_description:-TODO(api-verify)}
PAGE
        fi
        if [[ $has_constructor -eq 1 ]]; then
            cat <<PAGE

## Verified constructor signature

\`\`\`rust
${constructor_sig}
\`\`\`
PAGE
        fi
        if [[ $has_element -eq 1 ]]; then
            cat <<PAGE

## Verified element declaration

\`\`\`rust
${element_sig}
\`\`\`
PAGE
        fi
        render_example_section "Example References" "${ex_files[@]}"
        if [[ -n "${constructor_inline_examples//[[:space:]]/}" || -n "${element_inline_examples//[[:space:]]/}" ]]; then
            cat <<'PAGE'

## Inline Examples (from rustdoc)
PAGE
            if [[ -n "${constructor_inline_examples//[[:space:]]/}" ]]; then
                cat <<'PAGE'

### Constructor example
PAGE
                echo
                echo "$constructor_inline_examples"
            fi
            if [[ -n "${element_inline_examples//[[:space:]]/}" ]]; then
                cat <<'PAGE'

### Element example
PAGE
                echo
                echo "$element_inline_examples"
            fi
        fi
        cat <<'PAGE'

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
PAGE
    } > "$ROOT/families/${family_slug}.md"
done <<< "$family_slugs"

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

cat > "$ROOT/families.md" <<PAGE
---
title: Families
description: Unified widget-family pages that group module, constructor, and element APIs.
version: latest
last_updated: ${DATE}
order: 93
---

# Families

Generated from ref/doc/iced/widget/sidebar-items.js by normalizing related module/function/struct names into family slugs.

## Family Index

PAGE
while read -r family_slug || [[ -n "$family_slug" ]]; do
    [[ -z "$family_slug" ]] && continue
    echo "- [$(titleize_name "$family_slug")](/latest/reference/families/${family_slug})" >> "$ROOT/families.md"
done <<< "$family_slugs"

#!/usr/bin/env bash
set -euo pipefail

ROOT="src/content/latest/reference"
DATE="2026-02-19"

mkdir -p "$ROOT"

widget_sidebar="ref/doc/iced/widget/sidebar-items.js"
iced_sidebar="ref/doc/iced/sidebar-items.js"

extract_array() {
    local file="$1"
    local key="$2"
    sed -E "s/.*\"${key}\":\[([^]]+)\].*/\1/" "$file" | tr -d '"' | tr ',' '\n' | sed '/^$/d'
}

kebab_from_pascal() {
    local value="$1"
    echo "$value" | sed -E 's/([a-z0-9])([A-Z])/\1-\2/g' | tr '[:upper:]' '[:lower:]'
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
    if [[ "$fn_name" != "application" && "$fn_name" != "daemon" && "$fn_name" != "exit" && "$fn_name" != "never" && "$fn_name" != "run" ]]; then
        continue
    fi

    runtime_order=$((runtime_order + 1))
    file="$ROOT/runtime-fn-${fn_name}.md"

    cat > "$file" <<PAGE
---
title: Runtime Function - ${fn_name}
description: Detailed guidance for iced::${fn_name}.
version: latest
last_updated: ${DATE}
order: ${runtime_order}
---

# Runtime Function - iced::${fn_name}

This page documents when and why to use iced::${fn_name}, using ref/doc/iced/fn.${fn_name}.html as the authoritative API definition.

## What it does

Rustdoc source: ref/doc/iced/fn.${fn_name}.html.

## When to use it

- Use this function when its runtime behavior matches your application lifecycle needs.
- Prefer the simplest entrypoint that still gives you required hooks (title/theme/window/subscription/boot behavior).

## Why it exists

- It provides a focused runtime entrypoint in the iced API surface.
- It keeps application startup intent explicit and type-checked.

## Usage notes

- Validate exact signature and trait bounds in rustdoc before relying on advanced generic behavior.
- Cross-check with ref/examples for production-style usage patterns.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
PAGE

done < <(extract_array "$iced_sidebar" "fn")

# Module pages.
module_order=100
while read -r module_name || [[ -n "$module_name" ]]; do
    [[ -z "$module_name" ]] && continue
    module_order=$((module_order + 1))

    cat > "$ROOT/widget-module-${module_name}.md" <<PAGE
---
title: Widget Module - ${module_name}
description: Module-level reference for iced::widget::${module_name}.
version: latest
last_updated: ${DATE}
order: ${module_order}
---

# Widget Module - iced::widget::${module_name}

This page tracks the module entrypoint defined in ref/doc/iced/widget/${module_name}/index.html.

## Overview

- Module name: ${module_name}
- Rustdoc: ref/doc/iced/widget/${module_name}/index.html

## When to use

Use this module when you need the widget family and related styling/state APIs grouped under iced::widget::${module_name}.

## Common workflow

1. Start from the constructor function in iced::widget.
2. Use this module docs for detailed types, catalogs, and style options.
3. Cross-check with matching examples in ref/examples.

## Related

- [Widgets Overview](/latest/reference/widgets-overview)
- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
PAGE

done < <(extract_array "$widget_sidebar" "mod")

# Constructor pages.
constructor_order=300
while read -r fn_name || [[ -n "$fn_name" ]]; do
    [[ -z "$fn_name" ]] && continue
    constructor_order=$((constructor_order + 1))

    cat > "$ROOT/widget-constructor-${fn_name}.md" <<PAGE
---
title: Widget Constructor - ${fn_name}
description: Function reference for iced::widget::${fn_name}.
version: latest
last_updated: ${DATE}
order: ${constructor_order}
---

# Widget Constructor - iced::widget::${fn_name}

Authoritative source: ref/doc/iced/widget/fn.${fn_name}.html.

## What it returns

This function constructs a widget element (or helper wrapper) in the iced::widget namespace.

## When to use

- Use it as the primary constructor for this widget/helper.
- Chain builder methods on the returned widget value to configure behavior and style.

## Why it matters

Constructors keep UI trees explicit and strongly typed.

## API verification

Check exact generic parameters, argument types, and bounds in:

- ref/doc/iced/widget/fn.${fn_name}.html

## Related

- [Widget Modules Catalog](/latest/reference/widget-modules-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
PAGE

done < <(extract_array "$widget_sidebar" "fn")

# Element struct pages.
element_order=500
while read -r struct_name || [[ -n "$struct_name" ]]; do
    [[ -z "$struct_name" ]] && continue
    element_order=$((element_order + 1))
    slug="$(kebab_from_pascal "$struct_name")"

    cat > "$ROOT/widget-element-${slug}.md" <<PAGE
---
title: Widget Element - ${struct_name}
description: Struct reference for iced::widget::${struct_name}.
version: latest
last_updated: ${DATE}
order: ${element_order}
---

# Widget Element - iced::widget::${struct_name}

Authoritative source: ref/doc/iced/widget/struct.${struct_name}.html.

## Overview

${struct_name} is an element struct in the iced::widget API surface.

## When to use

Use this type when you need direct type-level control over the widget value returned by constructors/builders.

## Why this page exists

- To provide one reference page per exposed widget element type.
- To keep navigation complete and version-aware.

## API verification

Inspect exact methods and trait impls in:

- ref/doc/iced/widget/struct.${struct_name}.html

## Related

- [Widgets Overview](/latest/reference/widgets-overview)
- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
PAGE

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

All module pages are generated from ref/doc/iced/widget/sidebar-items.js.

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

All constructor pages are generated from ref/doc/iced/widget/sidebar-items.js.

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

All element pages are generated from ref/doc/iced/widget/sidebar-items.js.

## Elements

PAGE

while read -r struct_name || [[ -n "$struct_name" ]]; do
    [[ -z "$struct_name" ]] && continue
    slug="$(kebab_from_pascal "$struct_name")"
    echo "- [Widget Element - ${struct_name}](/latest/reference/widget-element-${slug})" >> "$ROOT/widget-elements-catalog.md"
done < <(extract_array "$widget_sidebar" "struct")

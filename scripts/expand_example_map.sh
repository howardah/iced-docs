#!/usr/bin/env bash
set -euo pipefail

MAP_FILE="src/docs-meta/example_map.tsv"
WIDGET_SIDEBAR="ref/doc/iced/widget/sidebar-items.js"
ICED_SIDEBAR="ref/doc/iced/sidebar-items.js"

mkdir -p "$(dirname "$MAP_FILE")"

tmp_triples="$(mktemp)"
trap 'rm -f "$tmp_triples"' EXIT

extract_array() {
    local file="$1"
    local key="$2"
    sed -E "s/.*\"${key}\":\[([^]]+)\].*/\1/" "$file" | tr -d '"' | tr ',' '\n' | sed '/^$/d'
}

kebab_from_pascal() {
    local value="$1"
    echo "$value" | sed -E 's/([a-z0-9])([A-Z])/\1-\2/g' | tr '[:upper:]' '[:lower:]'
}

record() {
    local kind="$1"
    local id="$2"
    local file="$3"
    [[ -z "$kind" || -z "$id" || -z "$file" ]] && return
    echo -e "${kind}\t${id}\t${file}" >> "$tmp_triples"
}

contains_call() {
    local file="$1"
    local fn="$2"
    rg -q "(^|[^[:alnum:]_])${fn}[[:space:]]*\\(" "$file"
}

contains_path() {
    local file="$1"
    local name="$2"
    rg -q "widget::${name}([[:space:]:,>]|$)" "$file"
}

contains_type() {
    local file="$1"
    local ty="$2"
    rg -q "(^|[^[:alnum:]_])${ty}([<:{[:space:]]|$)" "$file"
}

# Merge existing mappings first so curated additions are preserved.
if [[ -f "$MAP_FILE" ]]; then
    while IFS=$'\t' read -r kind id examples; do
        [[ -z "${kind:-}" || "${kind:0:1}" == "#" ]] && continue
        IFS=',' read -ra paths <<< "$examples"
        for path in "${paths[@]}"; do
            path="$(echo "$path" | sed 's/^ *//; s/ *$//')"
            [[ -n "$path" ]] && record "$kind" "$id" "$path"
        done
    done < "$MAP_FILE"
fi

mapfile -t runtime_fns < <(extract_array "$ICED_SIDEBAR" "fn")
mapfile -t modules < <(extract_array "$WIDGET_SIDEBAR" "mod")
mapfile -t constructors < <(extract_array "$WIDGET_SIDEBAR" "fn")
mapfile -t element_structs < <(extract_array "$WIDGET_SIDEBAR" "struct")

# Constructor -> module hints for coverage inference.
declare -A ctor_module=(
    [bottom]=container
    [bottom_center]=container
    [bottom_right]=container
    [center]=container
    [center_x]=container
    [center_y]=container
    [right]=container
    [right_center]=container
    [container]=container
    [button]=button
    [checkbox]=checkbox
    [combo_box]=combo_box
    [float]=float
    [grid]=grid
    [keyed_column]=keyed
    [pane_grid]=pane_grid
    [pick_list]=pick_list
    [progress_bar]=progress_bar
    [radio]=radio
    [row]=row
    [scrollable]=scrollable
    [sensor]=sensor
    [shader]=shader
    [slider]=slider
    [space]=space
    [table]=table
    [text]=text
    [span]=text
    [rich_text]=text
    [value]=text
    [text_editor]=text_editor
    [text_input]=text_input
    [themer]=theme
    [toggler]=toggler
    [tooltip]=tooltip
    [vertical_slider]=vertical_slider
)

# Constructor -> element hints.
declare -A ctor_element=(
    [bottom]=container
    [bottom_center]=container
    [bottom_right]=container
    [center]=container
    [center_x]=container
    [center_y]=container
    [right]=container
    [right_center]=container
    [container]=container
    [button]=button
    [checkbox]=checkbox
    [column]=column
    [combo_box]=combo-box
    [float]=float
    [grid]=grid
    [mouse_area]=mouse-area
    [pane_grid]=pane-grid
    [pick_list]=pick-list
    [pin]=pin
    [progress_bar]=progress-bar
    [radio]=radio
    [responsive]=responsive
    [row]=row
    [scrollable]=scrollable
    [sensor]=sensor
    [shader]=shader
    [slider]=slider
    [space]=space
    [stack]=stack
    [text_editor]=text-editor
    [text_input]=text-input
    [themer]=themer
    [toggler]=toggler
    [tooltip]=tooltip
    [vertical_slider]=vertical-slider
)

while IFS= read -r file; do
    # Runtime functions.
    for fn in "${runtime_fns[@]}"; do
        if contains_call "$file" "iced::${fn}"; then
            record runtime "$fn" "$file"
        fi
    done

    # Constructors + inferred module/element coverage.
    for ctor in "${constructors[@]}"; do
        if contains_call "$file" "$ctor" || contains_call "$file" "widget::${ctor}"; then
            record constructor "$ctor" "$file"
            if [[ -n "${ctor_module[$ctor]:-}" ]]; then
                record module "${ctor_module[$ctor]}" "$file"
            fi
            if [[ -n "${ctor_element[$ctor]:-}" ]]; then
                record element "${ctor_element[$ctor]}" "$file"
            fi
        fi
    done

    # Explicit module usage.
    for module in "${modules[@]}"; do
        if contains_path "$file" "$module"; then
            record module "$module" "$file"
        fi
    done

    # Explicit element type usage.
    for ty in "${element_structs[@]}"; do
        if contains_type "$file" "$ty"; then
            record element "$(kebab_from_pascal "$ty")" "$file"
        fi
    done

done < <(find ref/examples -type f -name '*.rs' | sort)

# Build deterministic TSV map.
{
    echo "# kind<TAB>id<TAB>comma-separated example paths"
    echo "# kind values: runtime | module | constructor | element"

    sort -u "$tmp_triples" \
        | awk -F'\t' '
            {
                key = $1 FS $2
                if (paths[key] == "") paths[key] = $3
                else paths[key] = paths[key] "," $3
            }
            END {
                for (key in paths) print key FS paths[key]
            }
        ' \
        | LC_ALL=C sort -t$'\t' -k1,1 -k2,2
} > "$MAP_FILE"

echo "Updated $MAP_FILE"

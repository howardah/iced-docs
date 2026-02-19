#!/usr/bin/env bash
set -euo pipefail

# Format fenced Rust code blocks (```rust / ```rs) in markdown files.
# Usage:
#   scripts/rustfmt_markdown_code_blocks.sh
#   scripts/rustfmt_markdown_code_blocks.sh --check
#   scripts/rustfmt_markdown_code_blocks.sh path/to/file.md [...]

CHECK_ONLY=0

if [[ "${1:-}" == "--check" ]]; then
  CHECK_ONLY=1
  shift
fi

if ! command -v rustfmt >/dev/null 2>&1; then
  echo "error: rustfmt not found in PATH" >&2
  exit 1
fi

format_block_file() {
  local input_file="$1"
  local output_file="$2"

  # rustfmt can fail for partial snippets; keep original in that case.
  if rustfmt --emit stdout --edition 2021 "$input_file" >"$output_file" 2>/dev/null; then
    return 0
  fi

  cat "$input_file" >"$output_file"
  return 1
}

process_markdown_file() {
  local file="$1"
  local tmp_output
  tmp_output="$(mktemp)"

  local in_block=0
  local open_fence=""
  local open_line=""
  local code_tmp=""
  local block_ok=1

  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ $in_block -eq 0 ]]; then
      if [[ "$line" =~ ^(\`\`\`+)[[:space:]]*(rust|rs)[[:space:]]*$ ]]; then
        in_block=1
        open_fence="${BASH_REMATCH[1]}"
        open_line="$line"
        code_tmp="$(mktemp)"
        continue
      fi

      printf '%s\n' "$line" >>"$tmp_output"
      continue
    fi

    if [[ "$line" =~ ^${open_fence}[[:space:]]*$ ]]; then
      local formatted_tmp
      formatted_tmp="$(mktemp)"
      if ! format_block_file "$code_tmp" "$formatted_tmp"; then
        block_ok=0
      fi

      printf '%s\n' "$open_line" >>"$tmp_output"
      cat "$formatted_tmp" >>"$tmp_output"
      printf '%s\n' "$line" >>"$tmp_output"

      rm -f "$formatted_tmp" "$code_tmp"
      code_tmp=""
      open_fence=""
      open_line=""
      in_block=0
      continue
    fi

    printf '%s\n' "$line" >>"$code_tmp"
  done <"$file"

  # Unclosed fence: leave captured block untouched.
  if [[ $in_block -eq 1 ]]; then
    printf '%s\n' "$open_line" >>"$tmp_output"
    cat "$code_tmp" >>"$tmp_output"
    rm -f "$code_tmp"
  fi

  if cmp -s "$file" "$tmp_output"; then
    rm -f "$tmp_output"
    return 2
  fi

  if [[ $CHECK_ONLY -eq 1 ]]; then
    rm -f "$tmp_output"
    return 3
  fi

  mv "$tmp_output" "$file"
  if [[ $block_ok -eq 0 ]]; then
    return 1
  fi
  return 0
}

declare -a files=()
if [[ $# -gt 0 ]]; then
  files=("$@")
else
  while IFS= read -r path; do
    files+=("$path")
  done < <(rg --files -g '*.md')
fi

if [[ ${#files[@]} -eq 0 ]]; then
  echo "No markdown files found."
  exit 0
fi

changed=0
unchanged=0
partial=0

for file in "${files[@]}"; do
  if [[ ! -f "$file" ]]; then
    echo "skip: $file (not a file)" >&2
    continue
  fi

  if process_markdown_file "$file"; then
    rc=0
  else
    rc=$?
  fi
  case "$rc" in
    0)
      changed=$((changed + 1))
      if [[ $CHECK_ONLY -eq 0 ]]; then
        echo "formatted: $file"
      fi
      ;;
    1)
      changed=$((changed + 1))
      partial=$((partial + 1))
      if [[ $CHECK_ONLY -eq 0 ]]; then
        echo "formatted with fallback: $file"
      fi
      ;;
    2)
      unchanged=$((unchanged + 1))
      ;;
    3)
      changed=$((changed + 1))
      echo "would format: $file"
      ;;
    *)
      echo "error: failed processing $file" >&2
      exit 1
      ;;
  esac
done

echo "summary: changed=$changed unchanged=$unchanged fallback_blocks=$partial"

if [[ $CHECK_ONLY -eq 1 && $changed -gt 0 ]]; then
  exit 1
fi

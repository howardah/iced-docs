#!/usr/bin/env bash
set -euo pipefail

# Format fenced Rust code blocks (```rust / ```rs) in markdown files.
# Usage:
#   scripts/rustfmt_markdown_code_blocks.sh
#   scripts/rustfmt_markdown_code_blocks.sh --check
#   scripts/rustfmt_markdown_code_blocks.sh -r path/to/dir
#   scripts/rustfmt_markdown_code_blocks.sh [-r] path/to/file.md [...]

CHECK_ONLY=0
RECURSIVE=0
declare -a roots=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --check)
      CHECK_ONLY=1
      shift
      ;;
    -r|--recursive)
      RECURSIVE=1
      shift
      ;;
    --)
      shift
      while [[ $# -gt 0 ]]; do
        roots+=("$1")
        shift
      done
      break
      ;;
    -*)
      echo "error: unknown flag: $1" >&2
      exit 2
      ;;
    *)
      roots+=("$1")
      shift
      ;;
  esac
done

if ! command -v rustfmt >/dev/null 2>&1; then
  echo "error: rustfmt not found in PATH" >&2
  exit 1
fi

format_block_file() {
  local input_file="$1"
  local output_file="$2"

  # Use stdin/stdout to avoid rustfmt emitting file-path headers for temp files.
  # rustfmt can still fail for partial snippets; keep original in that case.
  if rustfmt --emit stdout --edition 2021 <"$input_file" >"$output_file" 2>/dev/null; then
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

list_markdown_recursive() {
  local root="$1"
  if command -v rg >/dev/null 2>&1; then
    if [[ "$root" == "." ]]; then
      rg --files -g '*.md'
    else
      rg --files "$root" -g '*.md'
    fi
  else
    find "$root" -type f -name '*.md'
  fi
}

list_markdown_non_recursive() {
  local root="$1"
  find "$root" -maxdepth 1 -type f -name '*.md'
}

declare -a files=()

if [[ ${#roots[@]} -eq 0 ]]; then
  while IFS= read -r path; do
    [[ -n "$path" ]] && files+=("$path")
  done < <(list_markdown_recursive "." | sort -u)
else
  list_file="$(mktemp)"
  trap 'rm -f "$list_file"' EXIT

  for root in "${roots[@]}"; do
    if [[ -f "$root" ]]; then
      [[ "$root" == *.md ]] && printf '%s\n' "$root" >>"$list_file"
      continue
    fi

    if [[ -d "$root" ]]; then
      if [[ $RECURSIVE -eq 1 ]]; then
        list_markdown_recursive "$root" >>"$list_file"
      else
        list_markdown_non_recursive "$root" >>"$list_file"
      fi
      continue
    fi

    echo "skip: $root (not found)" >&2
  done

  while IFS= read -r path; do
    [[ -n "$path" ]] && files+=("$path")
  done < <(sort -u "$list_file")
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

#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# The original script tried to check for posts/.nojekyll with a string-length test that is
# always false, so we keep the same no-op to preserve behavior.
if [ -z 'posts/.nojekyll' ]; then
  echo "post directory must contain a .nojekyll file to disable jekyll SEO"
  touch posts/.nojekyll
fi

extract_subpath() {
  local value
  value="$(perl -ne 'if (/pub\s+const\s+SUBPATH.*"([^"]*)"/) { print $1; exit }' src/constant.rs 2>/dev/null || true)"
  if [ -z "${value:-}" ]; then
    value="/"
  fi
  printf '%s' "$value"
}

raw_subpath="$(extract_subpath)"
path="$(printf '%s' "$raw_subpath" | tr -d '/";[:space:]')"
target="$path"

if [ -n "$path" ]; then
  echo "Compiled With Sub-Path: $path"
fi

base_tag_update() {
  local replacement
  if [ -n "$target" ]; then
    replacement="<base data-trunk-public-url /${target}/>"
  else
    replacement="<base data-trunk-public-url />"
  fi

  if [ -f index.html ]; then
    perl -0pi -e 's@<base\s+data-trunk-public-url\b[^>]*>@'"$replacement"'@' index.html
  fi
}

base_tag_update

if [ -n "$path" ]; then
  rm -rf "$target" dist/
  trunk build --public-url "$path" --release
  mv dist "$target"
  touch "$target/.nojekyll"
else
  trunk build --release
fi

if [ "${1:-}" = "--gitpage" ]; then
  echo "compiled for Github Pages at docs/"
  rm -rf docs
  if [ -n "$path" ]; then
    mv "$target" docs
  else
    mv dist docs
  fi
fi

exit 0

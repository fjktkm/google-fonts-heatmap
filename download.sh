#!/usr/bin/env bash
set -e

ROOT_DIR="fonts"
URL="https://github.com/google/fonts.git"
REF="main"

mkdir -p "$ROOT_DIR"
git clone --filter=blob:none --no-checkout "$URL" "$ROOT_DIR"
cd "$ROOT_DIR"
git sparse-checkout init --no-cone
git sparse-checkout set -- \
    "apache/*/*.ttf" \
    "ofl/*/*.ttf" \
    "ufl/*/*.ttf" \
    "!ofl/adobeblank/AdobeBlank-Regular.ttf"
git fetch origin "$REF" --depth=1 --filter=blob:none
git switch --detach FETCH_HEAD

#!/usr/bin/env bash
set -e

ROOT_DIR="data/google/fonts"
URL="https://github.com/google/fonts.git"

git clone --depth=1 "$URL" "$ROOT_DIR"

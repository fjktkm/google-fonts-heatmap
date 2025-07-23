#!/usr/bin/env bash

target=fonts
url="https://github.com/google/fonts/archive/main.tar.gz"

mkdir -p "$target"
curl -L --progress-meter "$url" \
  | tar -xz --strip-components=1 -C "$target"

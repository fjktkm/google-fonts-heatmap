#!/usr/bin/env bash

commit=6526fad630c078afb8bfb134f2efc77f2ccd7d17
target=fonts
url="https://github.com/google/fonts/archive/${commit}.tar.gz"

mkdir -p "$target"
curl -L --progress-meter "$url" \
  | tar -xz --strip-components=1 -C "$target"

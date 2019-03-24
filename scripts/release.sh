#!/bin/bash

set -eu

# TODO: get cross working for cross-compilation

TARGETS=(
  x86_64-unknown-linux-gnu
  # x86_64-apple-darwin
)

mkdir -p /tmp/artifacts

for target in "${TARGETS[@]}"; do
  rustup target add "$target"
  cargo build --release --target "$target"
  # cross build --release --target "$target"
  tar czf  "/tmp/artifacts/resufancy-${target}.tar.gz" --directory "target/${target}/release" resufancy
done

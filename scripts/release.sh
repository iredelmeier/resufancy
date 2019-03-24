#!/bin/bash

set -eu

TARGETS=(
  x86_64-unknown-linux-gnu
  x86_64-apple-darwin
)
mkdir -p /tmp/artifacts

for target in "${TARGETS[@]}"; do
  rustup target add "$target"
  cross build --release --target "$target"
  tar czf  "/tmp/artifacts/resufancy-${target}.tar.gz" "target/${target}/release/resufancy"
done

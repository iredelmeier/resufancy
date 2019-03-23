#!/bin/bash

set -eu

cargo clippy --all -- \
  `# Exit with a nonzero code if there are clippy warnings` \
  -Dwarnings \
  `# Allow the unit_arg lint because a derive(Arbitrary) is causing a warning,` \
  `# and allow the lint for that line doesn't seem to work` \
  -A clippy::unit_arg \
  "$@"

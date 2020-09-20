#!/bin/bash
set -euo pipefail

cargo run --release --bin gbhwdb-deploy

# TARGET="s3://gbhwdb.gekkio.fi"
# CC_S="max-age=3600,public"
# CC_M="max-age=86400,public"
# CC_L="max-age=1209600,public"
#
# sync() {
#   aws --region eu-west-1 s3 sync "$@"
# }
#
# sync --cache-control "${CC_S}" --exclude "*.jpg" ./build/site/static/ "${TARGET}"/static/
# sync --cache-control "${CC_L}" --exclude "*" --include "*.jpg" ./build/site/static/ "${TARGET}"/static/
# sync --cache-control "${CC_M}" ./build/site/consoles/ "${TARGET}"/consoles/
# sync --cache-control "${CC_M}" ./build/site/cartridges/ "${TARGET}"/cartridges/
# sync --cache-control "${CC_S}" --exclude "static/*" --exclude "consoles/*" --exclude "cartridges/*" ./build/site/ "${TARGET}"/

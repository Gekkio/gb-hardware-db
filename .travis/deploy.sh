#!/bin/bash
set -euo pipefail

aws --region eu-west-1 s3 sync ./build/site/ s3://gbhwdb.gekkio.fi/

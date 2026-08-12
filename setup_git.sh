#!/bin/bash
set -euo pipefail
set -x

git submodule update --init --recursive
bash tools/checkout-submodule-branches.sh
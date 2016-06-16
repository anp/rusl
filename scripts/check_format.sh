#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..

cd ${BASE_DIR}

# check to see if any rusl failures are new failures
# we need to invert the exit code of grep here, if nothing's found it's a good thing

if ! rustfmt --write-mode diff src/lib.rs | grep -c "Diff at line"; then
    true
else
    echo "Failed style check."
    false
fi

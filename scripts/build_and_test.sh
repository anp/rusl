#!/usr/bin/env bash

set -ex

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld
SCRIPT_DIR=${BASE_DIR}/scripts

mkdir -p "${BUILD_DIR}"/usr
"${SCRIPT_DIR}"/musl.sh
"${SCRIPT_DIR}"/rusl.sh
"${SCRIPT_DIR}"/test_musl.sh
"${SCRIPT_DIR}"/frankenstein.sh
"${SCRIPT_DIR}"/test_rusl.sh

set +x
echo
echo
echo
echo
echo "Tests that failed on rusl but not on vanilla musl:"
echo

# check to see if any rusl failures are new failures
# we need to invert the exit code of grep here, if nothing's found it's a good thing
if ! grep -v -F -x -f "${BASE_DIR}"/musl_failures "${BASE_DIR}"/rusl_failures; then
    true
else
    false
fi

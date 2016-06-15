#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld
SCRIPT_DIR=${BASE_DIR}/scripts

mkdir -p ${BUILD_DIR}/usr
${SCRIPT_DIR}/musl.sh
${SCRIPT_DIR}/rusl.sh
${SCRIPT_DIR}/frankenstein.sh
${SCRIPT_DIR}/test.sh

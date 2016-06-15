#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld

cd ${BASE_DIR}
cargo build --release
cp target/release/librusl.a ${BUILD_DIR}/usr/lib

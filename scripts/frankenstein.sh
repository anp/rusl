#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld
MUSL_SRC_DIR=${BASE_DIR}/musl
TESTS_SRC_DIR=${BASE_DIR}/libc-test

# move the two archives into a tempdir
cd ${BUILD_DIR}/usr/lib
LIB_TEMP_DIR=${BUILD_DIR}/usr/lib/temp
mkdir ${LIB_TEMP_DIR}
mv librusl.a ${LIB_TEMP_DIR}/

# extract the object files
# and recombine them so that the Rust symbols are alongside the C symbols
cd ${LIB_TEMP_DIR}
ar -x librusl.a
ar -r ../libc.a ${LIB_TEMP_DIR}/*.o
rm -r ${LIB_TEMP_DIR}

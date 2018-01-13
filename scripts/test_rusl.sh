#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld
TESTS_SRC_DIR=${BASE_DIR}/libc-test
NUM_CPUS=$(grep -c ^processor /proc/cpuinfo)

cd "${TESTS_SRC_DIR}"

cp config.mak.def config.mak
echo "CFLAGS += -static -isystem ${BUILD_DIR}/usr/include -B${BUILD_DIR}/usr/librusl -L${BUILD_DIR}/usr/librusl" >> config.mak
echo "LDFLAGS += -static -isystem ${BUILD_DIR}/usr/include -B${BUILD_DIR}/usr/librusl -L${BUILD_DIR}/usr/librusl" >> config.mak
make -j "${NUM_CPUS}" CC="${BUILD_DIR}"/usr/bin/musl-gcc
grep FAIL < "${TESTS_SRC_DIR}"/src/REPORT > "${BASE_DIR}"/rusl_failures

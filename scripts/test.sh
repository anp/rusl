#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld
TESTS_SRC_DIR=${BASE_DIR}/libc-test
NUM_CPUS=`grep -c ^processor /proc/cpuinfo`

cd ${TESTS_SRC_DIR}

cp config.mak.def config.mak
echo "CFLAGS += -static -isystem ${BUILD_DIR}/usr/include -B${BUILD_DIR}/usr/lib -L${BUILD_DIR}/usr/lib" >> config.mak
echo "LDFLAGS += -static -isystem ${BUILD_DIR}/usr/include -B${BUILD_DIR}/usr/lib -L${BUILD_DIR}/usr/lib" >> config.mak
make -j ${NUM_CPUS} CC=${BUILD_DIR}/usr/bin/musl-gcc
cat ${TESTS_SRC_DIR}/src/REPORT | grep FAIL > ${BASE_DIR}/rusl_failures

echo
echo
echo
echo
echo "Tests that failed on rusl but not on vanilla musl:"
echo

cd ${BASE_DIR}

# check to see if any rusl failures are new failures
# we need to invert the exit code of grep here, if nothing's found it's a good thing
if ! grep -v -F -x -f ${BASE_DIR}/baseline_failures ${BASE_DIR}/rusl_failures; then
    true
else
    false
fi

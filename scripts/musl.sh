#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld
MUSL_SRC_DIR=${BASE_DIR}/musl
NUM_CPUS=`grep -c ^processor /proc/cpuinfo`

cd ${MUSL_SRC_DIR}
  ./configure --prefix=${BUILD_DIR}/usr \
      --disable-shared \
      --enable-static \
      --disable-visibility
make -j ${NUM_CPUS}
make install
cd ${BASE_DIR}

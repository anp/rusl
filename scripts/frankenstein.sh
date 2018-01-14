#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld

# move the two archives into a tempdir
cd "${BUILD_DIR}"/usr/lib
LIB_RUSL_DIR="${BUILD_DIR}"/usr/librusl
mkdir -p "${LIB_RUSL_DIR}"
cp "${BASE_DIR}"/target/release/librusl.a "${LIB_RUSL_DIR}"/
cp -t "${LIB_RUSL_DIR}"/ *.a *.specs # get .o files in a bit

cd "${LIB_RUSL_DIR}"

PORTED_OBJECTS=()
while IFS= read -r line; do
  PORTED_OBJECTS+=(${line})
done < "${BASE_DIR}"/ported_objects

# delete all ported object files from libc.a
ar -dv libc.a "${PORTED_OBJECTS[@]}"

# add in all .o files from librusl.a
ar -x librusl.a
ar -r libc.a *.o

rm *.o
# get remaining .o files from lib
cp -t ./ ../lib/*.o

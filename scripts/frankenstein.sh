#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR=${DIR}/..
BUILD_DIR=${BASE_DIR}/bld

# move the two archives into a tempdir
cd "${BUILD_DIR}"/usr/lib
LIB_TEMP_DIR="${BUILD_DIR}"/usr/lib/temp
mkdir "${LIB_TEMP_DIR}"
cp "${BASE_DIR}"/target/release/librusl.a "${LIB_TEMP_DIR}"/

cd "${LIB_TEMP_DIR}"
# delete all ported object files from libc.a
PORTED_OBJECTS=()
while IFS= read -r line; do
  PORTED_OBJECTS+=(${line})
done < "${BASE_DIR}"/ported_objects
ar -dv ../libc.a "${PORTED_OBJECTS[@]}"
ar -x librusl.a
ar -r ../libc.a "${LIB_TEMP_DIR}"/*.o
rm -r "${LIB_TEMP_DIR}"

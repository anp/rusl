#!/usr/bin/env bash

set -e

BASE_DIR=`pwd`
BUILD_DIR=${BASE_DIR}/build
MUSL_SRC_DIR=${BASE_DIR}/musl
TESTS_SRC_DIR=${BASE_DIR}/libc-test
NUM_CPUS=`grep -c ^processor /proc/cpuinfo`

clean_and_exit_build_dir() {
    cd ${BASE_DIR}
    rm -dr ${BUILD_DIR}
}

build_musl() {
    cd ${MUSL_SRC_DIR}
    ./configure --prefix=${BUILD_DIR}/usr \
        --disable-shared \
        --enable-static \
        --disable-visibility
    make -j ${NUM_CPUS}
    make install
    cd ${BASE_DIR}
}

build_rusl() {
    cd ${BASE_DIR}
    cargo build --release
    cp target/release/librusl.a ${BUILD_DIR}/usr/lib
}

combine_rusl_and_musl() {
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
    rm -dr ${LIB_TEMP_DIR}
}

build_and_run_tests() {
    cd ${TESTS_SRC_DIR}

    cp config.mak.def config.mak
    echo "CFLAGS += -static -isystem ${BUILD_DIR}/usr/include -B${BUILD_DIR}/usr/lib -L${BUILD_DIR}/usr/lib" >> config.mak
    echo "LDFLAGS += -static -isystem ${BUILD_DIR}/usr/include -B${BUILD_DIR}/usr/lib -L${BUILD_DIR}/usr/lib" >> config.mak
    make -j ${NUM_CPUS} CC=${BUILD_DIR}/usr/bin/musl-gcc
    cat ${TESTS_SRC_DIR}/src/REPORT | grep FAIL > ${BASE_DIR}/rusl_failures

    echo "#####################################################################"
    echo "#####################################################################"
    echo "#####################################################################"
    echo "#####################################################################"
    echo "Tests that failed on rusl but not on vanilla musl:"
    echo "#####################################################################"
    grep -v -F -x -f ${BASE_DIR}/baseline_failures ${BASE_DIR}/rusl_failures

    cd ${BASE_DIR}
}

#trap clean_and_exit_build_dir EXIT

mkdir -p ${BUILD_DIR}/usr
build_musl
build_rusl
combine_rusl_and_musl
build_and_run_tests

#!/usr/bin/env bash

BASE_DIR=`pwd`
BUILD_DIR=${BASE_DIR}/bld
MUSL_SRC_DIR=${BASE_DIR}/musl
TESTS_SRC_DIR=${BASE_DIR}/libc-test

rm -dr ${BUILD_DIR}
cd ${MUSL_SRC_DIR} && make clean
cd ${TESTS_SRC_DIR} && make cleanall
cd ${BASE_DIR} && cargo clean

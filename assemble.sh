#!/usr/bin/env bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=jtag

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

arm-none-eabi-as -g -march=armv6-m asm.s -o bin/$crate.o
ar crs bin/thumbv6m-none-eabi.a bin/$crate.o

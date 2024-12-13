#!/bin/bash

set -e
PROG_PREFIX="$1"

# Setup Rust
kani --keep-temps --only-codegen ${PROG_PREFIX}.rs
mv ${PROG_PREFIX}*foo.out rs_goto

rm ${PROG_PREFIX}*.json
rm ${PROG_PREFIX}*foo.*.out
rm *.rlib

# Setup C
goto-cc ${PROG_PREFIX}.c -o c_goto
goto-cc caller.c -o caller_goto
goto-instrument --show-goto-functions c_goto > c_goto.txt
goto-instrument --show-goto-functions caller_goto > caller_goto.txt

# Link
goto-cc c_goto caller_goto rs_goto -o main_goto
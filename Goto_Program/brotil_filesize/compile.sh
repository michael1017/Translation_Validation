PROG_PREFIX="$1"

# Setup Rust
kani --keep-temps --only-codegen ${PROG_PREFIX}.rs
mv ${PROG_PREFIX}*foo.out rs_goto

# Setup C
goto-cc ${PROG_PREFIX}.c -o c_goto
goto-cc caller.c -o caller_goto

# Link
goto-cc c_goto caller_goto rs_goto -o main_goto
goto-instrument --show-goto-functions main_goto > main_goto.txt
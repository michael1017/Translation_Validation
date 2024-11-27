PROG_PREFIX="$1"

# Setup Rust
kani --keep-temps ${PROG_PREFIX}.rs
mv ${PROG_PREFIX}*main.out rs_goto
goto-instrument --list-goto-functions rs_goto > rs_goto_func.txt

# Setup C
goto-cc ${PROG_PREFIX}.c -o c_goto
goto-cc caller.c -o caller_goto

# Link
goto-cc c_goto caller_goto rs_goto -o main_goto
goto-instrument --show-goto-functions main_goto > main_goto.txt
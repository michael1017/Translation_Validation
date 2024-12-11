## Changes
Remove main function in C

## Commands
sh compile.sh BINARY_SEARCH
cbmc --no-standard-checks main_goto

## Notes
Halt.
Recursion exists in program.

Return Error when --unwind 1
Reason:
function interface type not match

Works after add:
`__CPROVER_assume(l >= 0 && r >= 0 && arrLen >= 0);`
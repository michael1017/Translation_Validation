## Changes
Remove main function in C
Remove main function in Rust

In Rust:
fn f_filled(arr: &[i32; 4], n: usize, x: i32) -> i32
Setup arr length to 4

In C:
Change function interface 
int f_gold ( int arr [ ], unsigned long int n, int x )
this is required to align Rust usize.


## Commands
sh compile.sh ANALYSIS_OF_ALGORITHMS_SET_2_ASYMPTOTIC_ANALYSIS
cbmc --no-standard-checks main_goto

## Note
Halt.
While loop exists in program.
Works if add --unwind 100
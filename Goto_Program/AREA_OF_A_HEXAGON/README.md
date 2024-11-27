## Changes
Remove main function in C



## Commands
sh compile.sh AREA_OF_A_HEXAGON
cbmc --no-standard-checks main_goto

## Note
`sh compile.sh` doesn't work. Need to manually generate goto prog for Rust

res1=4.203895e-45f (00000000 00000000 00000000 00000011)
res2=2.802597e-45f (00000000 00000000 00000000 00000010)
get non-equal result
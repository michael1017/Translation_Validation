# Conversion to Goto Programs

## Prerequirements
* [Kani Compiler](https://model-checking.github.io/kani/install-guide.html#installing-the-latest-version)
* CBMC

## C to Goto Progarm
Run the following command
```
goto-cc sum.c -o c_goto
```

You should see the goto program `c_goto` in binary format. If you want to inspect it, run
```
goto-instrument --show-goto-functions c_goto
```

## Rust to Goto Program
Run the following command
```
kani --keep-temps --only-codegen sum.rs
``` 

You should see some output files like this. The file we want is `sum__RNvCsaks48gR9Qj8_3sum12kani_harness.out`
```
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.out
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.pretty_name_map.json  
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.symtab.out
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.type_map.json
```

Simply rename the Goto program we want into `rs_goto`
```
mv sum__RNvCsaks48gR9Qj8_3sum3foo.out rs_goto
```
## Verify Two Goto Programs
In the previous step, we two goto programs, `c_goto` and `rs_goto`. 

Now we need a compare function written in `caller.c` to verfiy two functions' equality. To verify, combine prevous goto programs produced from `caller.c` to generate a new goto program called `main_goto`.
```
goto-cc caller.c -o caller_goto
goto-cc c_goto caller_goto rs_goto -o main_goto
```

Then run CBMC
```
cbmc --no-standard-checks main_goto
```

If you see something like the following, it measn these two prgrams are varified to be equal.
```
[main.assertion.1] line 8 Programs are equal: SUCCESS
```
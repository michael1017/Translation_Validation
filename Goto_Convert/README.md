# Conversion to Goto Programs

## Prerequirements
* [Kani Compiler](https://model-checking.github.io/kani/install-guide.html#installing-the-latest-version)
* CBMC

## C to Goto Progarm
Run the following command
```
goto-cc sum1.c -o sum1
```

You should see the goto program `sum1` in binary format. If you want to inspect it, run
```
goto-instrument --show-goto-functions sum1
```

## Rust to Goto Program
Run the following command
```
kani --keep-temps sum.rs
``` 

You should see some output files like this. The file we want is `sum__RNvCsaks48gR9Qj8_3sum12kani_harness.out`
```
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.out
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.pretty_name_map.json  
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.symtab.out
sum__RNvCsaks48gR9Qj8_3sum12kani_harness.type_map.json
```

## Verify Two Goto Programs
In the previous step, we two goto programs, `sum1` and `sum__RNvCsaks48gR9Qj8_3sum12kani_harness.out`. 

Now we need a caller function written in `sum_main.c` to verfiy two functions' equality. To verify, combine prevous goto programs with `sum_main.c` to generate a new goto program called `sum_mix`.
```
goto-cc sum1 sum__RNvCsaks48gR9Qj8_3sum12kani_harness.out sum_main.c -o sum_mix
```

Then run CBMC
```
cbmc --no-standard-checks sum_mix
```


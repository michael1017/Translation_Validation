# Translation_Validation

## Folder Layout

### GOTO_Convert
This folder provides a basic example of comparing C and Rust program functions. For more details, see [LLM_Translation/README.md](LLM_Translation/README.md)

### Goto_program
This folder documents comparisons between original C programs in `LLM_Translation/C_programs` and Rust programs in `LLM_Translation/Rust_programs_compiled`. Subfolder README details the changes made to the source code, the process for generating goto programs, and the comparison results.

### LLV_Translation
This folder contains the original C programs in the `C_programs` subfolder. Translated Rust programs are in `Rust_programs_compiled` (if they compile) or `Rust_programs_not_compile` (if they do not). For more details, see [LLM_Translation/README.md](LLM_Translation/README.md)

### verification
This folder performs similar tasks to the `Goto_program` folder. It contains the Makefiles to generate and link the Goto programs for comparision.
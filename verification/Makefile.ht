# TODO: Update to your program directory
ROOT := /home/pamusuo/Fall2024/cs560/Translation_Validation/verification

# TODO: Update these variables
# Define variables for file paths (update these as needed)
C_FILE := $(ROOT)/ht.c
R_FILE := $(ROOT)/ht.rs
COMP_FILE := $(ROOT)/comp_prog_ht.c

# Directories
BUILD_DIR := $(ROOT)/build
GOTOS_DIR := $(BUILD_DIR)/gotos
REPORTS_DIR := $(BUILD_DIR)/reports

# Harness to utilize, omit the extension!
H_ENTRY = main

# Extra CBMC flags to be passed during CBMC analysis
H_CBMCFLAGS = --nondet-static --verbosity 9 --unwind 1 --unwindset hash_key_c.0:7 --unwindset hash_key_rust.0:7 --unwindset comp_main.0:7

# Extra header defintiions to be passed during compilation
H_DEF =

# Extra CBMC includes to be passed during compilation
H_INC = 


# Include our special build file
include ./Makefile.include

sources: $(GOTOS_DIR)/prog1.goto $(GOTOS_DIR)/comp_prog.goto $(GOTOS_DIR)/prog2.goto
	@echo "All source files are generated."

# Create directories if they don't exist
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(GOTOS_DIR): $(BUILD_DIR)
	mkdir -p $(GOTOS_DIR)
	mkdir -p $(REPORTS_DIR)

# Compile C file to goto program
$(GOTOS_DIR)/prog1.goto: $(GOTOS_DIR) $(C_FILE)
	goto-cc -o $@ $(C_FILE)

# Compile comparison file to goto program
$(GOTOS_DIR)/comp_prog.goto: $(GOTOS_DIR) $(COMP_FILE)
	goto-cc -o $@ $(COMP_FILE)

# Compile Rust file to goto program
# TODO: Sometimes, "main.out" need to be updated to main_rs.out. Review the file name of the kani-generated goto file for the correct name.
$(GOTOS_DIR)/prog2.goto: $(GOTOS_DIR) $(R_FILE)
	kani --keep-temps --only-codegen --target-dir $(GOTOS_DIR) $(R_FILE) > $(GOTOS_DIR)/prog2.txt 2>&1 
	find $(GOTOS_DIR) -name "*main.out" -exec mv {} $(GOTOS_DIR)/prog2.goto \; 


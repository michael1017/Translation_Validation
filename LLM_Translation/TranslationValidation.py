import shutil
from openai import OpenAI
import subprocess
import os
import time

class CtoRustTranslator:
    def __init__(self, input_path, output_path, api_key):
        self.input_path = input_path
        self.output_path = output_path
        self.api_key = api_key

    def process_c_file(self):
        """
        Reads C code from a file, translates it to Rust, and returns the result.
        """
        with open(self.input_path, 'r') as file:
            c_code = file.read()
            
        return c_code

    def save_rust_code(self, rust_code):
        """
        Saves the translated Rust code to a file.
        """
        with open(self.output_path, 'w') as file:
            file.write(rust_code)

    def translate_c_to_rust(self):
        c_code = self.process_c_file()
        client = OpenAI(api_key=self.api_key)
        completion = client.chat.completions.create(
            model="gpt-4o",
            messages=[
                {"role": "system", "content": "You are a helpful assistant. Please only output the code without code blocks."},
                {
                    "role": "user",
                    "content": "Translate the following C code to Rust: " + f" {c_code}"
                }
            ]
        )

        rust_code = completion.choices[0].message.content
        self.save_rust_code(rust_code)
    
    def check_rust_file_compiles(self):
        # Add the Rust installation directory to the PATH
        rust_path = os.environ.get("HOME") + "/.cargo/bin"  # Default Rust installation path for most systems
        os.environ["PATH"] += os.pathsep + rust_path

        try:
            # Compile the Rust file using rustc
            result = subprocess.run(
                ['rustc', '--crate-type', 'bin', self.output_path],
                capture_output=True,
                text=True,
                check=True
            )
            print(f"File {self.output_path} compiled successfully!")
            return True
        except subprocess.CalledProcessError as e:
            # If compilation fails, print the error
            print(f"Compilation failed for {self.output_path}.")
            print(f"Error message: {e.stderr}")
            return False
        
def move_failed_programs(compile_fail_programs, source_dir, dest_dir):
    # Create destination directory if it doesn't exist
    if not os.path.exists(dest_dir):
        os.makedirs(dest_dir)
            
    # Move each failed program to the destination directory
    for program in compile_fail_programs:
        source_path = os.path.join(source_dir, program)
        dest_path = os.path.join(dest_dir, program)
        try:
            shutil.move(source_path, dest_path)
            print(f"Moved {program} to {dest_dir}")
        except Exception as e:
            print(f"Error moving {program}: {str(e)}")

c_folder_path = "C_programs"
rust_folder_path = "Rust_programs_compiled"
# Create a directory for failed programs and move them there
failed_programs_dir = "Rust_programs_not_compiled"
api_key = "put_your_openai_api_key_here"

start_time = time.time()
compile_success_programs = []
compile_fail_programs = []
for c_file_name in os.listdir(c_folder_path):
    rs_file_name = os.path.splitext(c_file_name)[0] + ".rs"
    # Skip if Rust file already exists in either folder
    if (os.path.exists(os.path.join(rust_folder_path, rs_file_name)) or 
        os.path.exists(os.path.join(failed_programs_dir, rs_file_name))):
        print(f"Skipping {c_file_name} - Rust file already exists")
        continue

    input_path = os.path.join(c_folder_path, c_file_name)
    output_path = os.path.join(rust_folder_path, rs_file_name)
    ctoRustTranslator = CtoRustTranslator(input_path, output_path, api_key)
    ctoRustTranslator.translate_c_to_rust()
    compiled_successfully = ctoRustTranslator.check_rust_file_compiles()
    if (compiled_successfully):
        compile_success_programs.append(c_file_name)
    else:
        compile_fail_programs.append(c_file_name)

end_time = time.time()
elapsed_time = end_time - start_time
print(f"Elapsed time: {elapsed_time:.2f} seconds")

print(f"compile_success_programs: {compile_success_programs}")
print(f"compile_fail_programs: {compile_fail_programs}")

move_failed_programs(compile_fail_programs, c_folder_path, failed_programs_dir)



# # Check compilation of all existing Rust files
# print("\nChecking compilation of all Rust files...")
# rust_compile_success = []
# rust_compile_fail = []

# for rs_file_name in os.listdir(rust_folder_path):
#     if rs_file_name.endswith('.rs'):
#         input_path = "" # Not needed for just compilation check
#         output_path = os.path.join(rust_folder_path, rs_file_name)
#         rustValidator = CtoRustTranslator(input_path, output_path, api_key)
#         compiled_successfully = rustValidator.check_rust_file_compiles()
#         if compiled_successfully:
#             rust_compile_success.append(rs_file_name)
#         else:
#             rust_compile_fail.append(rs_file_name)

# print(f"\nExisting Rust files that compile successfully: {rust_compile_success}")
# print(f"Existing Rust files that fail to compile: {rust_compile_fail}")



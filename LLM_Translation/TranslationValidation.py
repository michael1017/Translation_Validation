import shutil
from openai import OpenAI
import subprocess
import os
import time
import ollama
from dotenv import load_dotenv

class CtoRustTranslator:
    def __init__(self, input_path, output_path, api_key, model):
        self.input_path = input_path
        self.output_path = output_path
        self.api_key = api_key
        self.model = model

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

    def translate_c_to_rust(self, error_message=""):
        c_code = self.process_c_file()
        rust_code = ""

        messages = [
                    {"role": "system", "content": "You are a helpful assistant. Strictly output Rust code only without any text description or code blocks like \"```rust\" or any other formatting."},
                    {
                        "role": "user",
                        "content": "Translate the following C code to Rust: " + f" {c_code}"
                    }
                ]
        if error_message != "":
            messages = [
                    {"role": "system", "content": "You are a helpful assistant. Strictly output Rust code only without any text description or code blocks like \"```rust\" or any other formatting."},
                    {
                        "role": "user",
                        "content": "Translated rust code has the following compilation errors: " + f" {error_message}. \n" + "Please try translating the following C code to Rust again: " + f" {c_code}"
                    }
                ]
            
        if model == "gpt-4o":
            client = OpenAI(api_key=self.api_key)
            completion = client.chat.completions.create(
                model="gpt-4o",
                messages=messages
            )

            rust_code = completion.choices[0].message.content
        else:
            response = ollama.chat(
                model='llama3.1:latest', 
                messages=messages
                )
            rust_code = response['message']['content']
        self.save_rust_code(self.llm_output_post_process(rust_code))

    def llm_output_post_process(self, rust_code):
        rust_code = rust_code.replace("```rust", "")
        rust_code = rust_code.replace("```", "")
        return rust_code
    
    def check_rust_file_compiles(self):
        # Add the Rust installation directory to the PATH
        rust_path = os.environ.get("HOME") + "/.cargo/bin"  # Default Rust installation path for most systems
        os.environ["PATH"] += os.pathsep + rust_path

        try:
            # Compile the Rust file using rustc in the /build folder
            result = subprocess.run(
                ['rustc', '--error-format=short', '-A','warnings', '--crate-type', 'bin', os.path.join('..', self.output_path)],
                capture_output=True,
                text=True,
                check=True,
                cwd='build/'
            )
            print(f"File {self.output_path} compiled successfully!")
            return True, ""
        except subprocess.CalledProcessError as e:
            # If compilation fails, print the error
            print(f"Compilation failed for {self.output_path}.")
            print(f"Error message: {e.stderr}")
            return False, e.stderr

c_folder_path = "C_programs"
rust_folder_path = "Rust_programs_compiled"
# Create a directory for failed programs and move them there
failed_programs_dir = "Rust_programs_not_compiled"

# Load environment variables from .env file
load_dotenv()
api_key = os.environ.get("OPENAI_API_KEY")
model = os.environ.get("MODEL")
if model is None:
    print("Error: MODEL is not specified. Please set the MODEL environment variable.")
    exit(1)
elif model == "gpt-4o" and api_key is None:
    print("Error: OPENAI_API_KEY is not specified. Please set the OPENAI_API_KEY environment variable.")
    exit(1)

start_time = time.time()
compile_success_programs = []
compile_fail_programs = []
for c_file_name in os.listdir(c_folder_path):
    # Skip non-.c files
    if not c_file_name.endswith('.c'):
        continue

    rs_file_name = os.path.splitext(c_file_name)[0] + ".rs"
    # Skip if Rust file already exists in Rust_programs_compiled folder
    if os.path.exists(os.path.join(rust_folder_path, rs_file_name)):
        print(f"Skipping {c_file_name} - Rust file already exists")
        continue
    print(f"Translating {c_file_name} to Rust...")
    input_path = os.path.join(c_folder_path, c_file_name)
    output_path = os.path.join(rust_folder_path, rs_file_name)
    ctoRustTranslator = CtoRustTranslator(input_path, output_path, api_key, model)
    ctoRustTranslator.translate_c_to_rust()
    compiled_successfully, error_message = ctoRustTranslator.check_rust_file_compiles()
    if (compiled_successfully):
        compile_success_programs.append(rs_file_name)
    else:
        num_llm_self_optimization = 3
        while num_llm_self_optimization < 3:
            print(f"self_optimization step {num_llm_self_optimization}: ")
            # reprompt llm to fix compilation error
            ctoRustTranslator.translate_c_to_rust(error_message)
            compiled_successfully, new_error_message = ctoRustTranslator.check_rust_file_compiles()
            if (compiled_successfully):
                compile_success_programs.append(rs_file_name)
                break
            error_message = new_error_message
            num_llm_self_optimization+=1
        if (compiled_successfully == False):
            compile_fail_programs.append(rs_file_name)
            

end_time = time.time()
elapsed_time = end_time - start_time
print(f"Elapsed time: {elapsed_time:.2f} seconds")

print(f"compile_success_programs: {compile_success_programs}")
print(f"compile_fail_programs: {compile_fail_programs}")

def move_failed_programs(compile_fail_programs, source_dir, dest_dir):
    # Create destination directory if it doesn't exist
    if not os.path.exists(dest_dir):
        os.makedirs(dest_dir)
            
    # Move each failed program to the destination directory
    for program in compile_fail_programs:
        source_path = os.path.join(source_dir, program)
        dest_path = os.path.join(dest_dir, program)
        # If file exists in destination, remove it before moving
        if os.path.exists(dest_path):
            os.remove(dest_path)
            print(f"Removed existing {program} from destination")

        try:
            shutil.move(source_path, dest_path)
            print(f"Moved {program} to {dest_dir}")
        except Exception as e:
            print(f"Error moving {program}: {str(e)}")

move_failed_programs(compile_fail_programs, rust_folder_path, failed_programs_dir)

def delete_duplicate_programs(compiled_dir, not_compiled_dir):
    """
    Delete files from not_compiled_dir if they already exist in compiled_dir
    
    Args:
        compiled_dir (str): Path to directory with compiled Rust programs
        not_compiled_dir (str): Path to directory with not compiled Rust programs
    """
    # Get list of files in compiled directory
    compiled_files = set(os.listdir(compiled_dir))
    
    # Check each file in not_compiled directory
    for filename in os.listdir(not_compiled_dir):
        if filename in compiled_files:
            file_path = os.path.join(not_compiled_dir, filename)
            try:
                os.remove(file_path)
                print(f"Deleted duplicate file {filename} from {not_compiled_dir}")
            except Exception as e:
                print(f"Error deleting {filename}: {str(e)}")

# Delete duplicates from not_compiled directory
delete_duplicate_programs(rust_folder_path, failed_programs_dir)



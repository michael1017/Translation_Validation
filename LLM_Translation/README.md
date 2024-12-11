# C to Rust Translator

## Overview

This program translates C code into Rust code using OpenAI's language models or Ollama's models. It reads C files from a specified directory, translates them, and attempts to compile the resulting Rust code. If compilation fails, the program will attempt to re-translate the C code with the provided error messages.

## Features

- Reads C code from a specified input directory.
- Translates C code to Rust using OpenAI's GPT-4o or Ollama's models.
- Saves the translated Rust code to a specified output directory.
- Compiles the Rust code and checks for compilation errors.
- Attempts to re-translate C code if compilation fails, using error messages to guide the translation.

## Requirements

- Python 3.x
- `openai` library (for OpenAI API)
- `ollama` library (for Ollama API)
- Rust installed on your system (with `rustc` available in your PATH)

## Installation

1. Install the required Python packages:
   ```bash
   pip install python-dotenv openai ollama
   ```

2. Set up your environment variables in a `.env` file:
   ```plaintext
   OPENAI_API_KEY="your_openai_api_key" # needed for gpt-4o
   MODEL="gpt-4o"  # or "llama3.1:latest"
   ```

3. If using Ollama, ensure Ollama is installed and follow these steps:
   - Pull the Ollama model using `ollama pull llama3.1:latest`.
   - Serve Ollama using `ollama serve`.

## Usage

1. Place your C files in the `C_programs` directory.
2. Run the `TranslationValidation.py` script:
   ```bash
   python3 LLM_Translation/TranslationValidation.py
   ```

3. The translated Rust files will be saved in the `Rust_programs_compiled` directory.
4. Any files that fail to compile will be moved to the `Rust_programs_not_compiled` directory.

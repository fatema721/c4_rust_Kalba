# C4 Rust Compiler

C4 is a simple Rust C compiler and interpreter that reads C code.

## Installation

Build a Cargo:

```bash
cargo build
```
Copy the main.rs and paste it into the main.rs in the src folder, download the test.c file and place it in the Cargo folder

## Usage

1. **Download test.c File**  


2. **Run the Code**  

Run C file:
   ```bash
   cargo run -- test.c
   ```
Source code:
   ```bash
   cargo run -- -s test.c
   ```
Debug info:
   ```bash
   cargo run -- test.c
   ```

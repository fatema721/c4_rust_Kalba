c4 - C in Rust
An exercise in minimalism.
C compiler in Rust
Clone the repository:
git clone https://github.com/yourusername/c4-rust.git
cd c4-rust

Configure Cargo.toml :
[package]
name = "c4"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

Build the executable Cargo file:
cargo build --release

Try the following:
./target/release/c4 test.c
./target/release/c4 -s test.c
./target/release/c4 -d test.c

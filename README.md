# c4 - C in Rust
========================

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
     cargo run -- test.c
     cargo run -- -s test.c
     cargo run -- -d test.c
    

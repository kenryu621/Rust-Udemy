# Rust-Udemy

Rust projects for the Rust Programming Language class in Udemy

## Cargo Package Manager

- `cargo new <project_name>` will generate a new Rust project folder.
	- Adding an argument `--bin` stands for binary, means the project is for creating an executable file instead of make a library.
- Source files should be in the `src` folder.
	- The file contained the `main` function should named `main.rs`.
- A `Cargo.toml` file is expected in the root directory.
- `cargo build` will use the files to compile and build an executable file.
- `cargo run` automatically compile, build, and run an executable file.
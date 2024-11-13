# [Rust: The Complete Developer's Guide](https://www.udemy.com/course/rust-the-complete-developers-guide)

## Installation

- Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - [Installation Link](https://www.rust-lang.org/tools/install)
- Check if rust was installed properly: `cargo --version`
- Create a new project: `cargo new <project-name>`
- Run a new project: `cargo run` (You need to be inside a rust working directory)
  - Use `-q` flag for quite mode
- Enable Intellisense: `rust-analyzer` extension in VS Code
- Enable Auto-formatting: Modify User JSON

  ```json
  "[rust]": {
    "editor.foldingStrategy": "indentation",
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  ```

- Crate is a Package in Rust
  - Installation: `cargo add <crate-name>`
    - Ex: `cargo add rand`
  - [Listing](crates.io)
  - [Docs](docs.rs)
  - Remove a crate
    - Remove/comment crate from `Cargo.toml`
    - Run command: `cargo update` => Crate removed

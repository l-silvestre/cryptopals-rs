# cryptopals-rs
Rust implementation of the Crypto pals challenge - https://cryptopals.com/

## Project Structure
```bash
├── data/ # stores all downloaded challenge files
│   └── ex<i>.txt # i will be exercise number
├── src/
│   ├── exercises/ # exercises module
│   │   ├── mod.rs # mod file to export exercise functions
│   │   └── setX.rs # exercise files, one file per set
│   ├── helpers/ # helpers module
│   │   ├── mod.rs # mod file to export helper functions
│   │   └── *.rs # helper function files
│   └── main.rs # rust project entrypoint
├── target/ # untracked folder; will store build artifacts
├── .gitignore # gitignore file with template rust ignored options
├── Cargo.lock # untracked file; lock file for installed dependencies
├── Cargo.toml # Project dependencies and metadata
└── README.md # Readme file
```

## Installation
This project requires rust to run. Please head to [rust's installation guide](https://doc.rust-lang.org/book/ch01-00-getting-started.html) for instruction on how to install.

## Running
To run project please use:
```sh
cargo run
```

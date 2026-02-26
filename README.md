# rust-fundamentals
Logging my journey of learning Rust from scratch.

Rust Version: rustc 1.93.0 (254b59607 2026-01-19)

## RUST INSTALLATION
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Troubleshoot Installation
```
rustc --version
```

## Updating Rust / Managing Rust Versions
```
rustup update
```
## Uninstallation
```
rustup self uninstall
```
# Cargo: Rust's Build System and Package Manager
1. Builds your code
2. Downloads libraries your code depends on
3. Use this strictly for managing your Rust projects

## Checking Cargo Version
```
cargo --version
```
Cargo Version: cargo 1.93.0 (083ac5135 2025-12-15)

## Creating a Project with Cargo
```
cargo new <your-project-name>
```

## Build and Run a Cargo Project
Build: Creates the binary / executable
You should be in the same directory as of the .toml
```
cargo build
```
Executable path:rust-.../target/debug

##Running the Executable
```
./target/debug/<your-project-name>
```
Quick Run the Build + Execution Step together, (Initially you'll love this more)
```
cargo run
```

## Verifying your code compiles
Do this everytime you change your source code 
```
cargo check
```



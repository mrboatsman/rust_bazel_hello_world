# Bazel Rust example workspace

The following example is a minimal example of a simple rust project consisting 
of a binary, library and external crates. This can be handled by Bazel.


# Instructions
1. Clone his repo 
2. Change the target in `Cargo.toml` under `[raze]` to fit your target

    Mac OSX: `target = "x86_64-apple-darwin"` 

    Linux 64-bit: `target = "x86_64-unknown-linux-gnu"`

    Windows 64-bit `target = "86_64-pc-windows-msvc"` 
    
3. `cargo install cargo-raze`
4. `cargo generate-lockfile` only needed when there is no Cargo.lock file
5. `cargo raze`
6. `bazel build :hello_world`
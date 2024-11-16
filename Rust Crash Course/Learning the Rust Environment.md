```
cargo --version
cargo new playground
cd playground
```

# 6. Cargo New

```rust
cargo add tokio --features full
```

Adds dependencies in Cargo.toml

```rust
cargo build
cargo run
```

Adds folder **target*r -> **debug** where is our **playground.exe**

`cargo build --release` created a **release** folder with a final **playground file**. We can now run this file.
`./target/release/playground.exe

**main.rs** is our entry point. The program will know, where to start. So, we can create different folders and files and functions in those files and call them from where ever
but the entire program needs to know, where do I start.

`cargo update` will update our dependencies.
`cargo fmt` formats our code

# 7. Your First Rust Function

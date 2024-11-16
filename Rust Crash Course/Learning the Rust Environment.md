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

add function 

```rust 
fn add_five
```
with an unsigned integer u32 (32 bits (4 bytes))

```rust
fn add_five(num: u32) 
```
and it`s going to return back an unsigned integer u32 and now we have an legitimate function

```rust
fn add_five(num: u32) -> u32 {

}
```
Rust is fantastic at ensuring that our code will work before I even go and run it. Rust makes you, forces you to get it right upfront.

Most developers get really frustrated with the borrow checker and compiler whether or not it is going to compile.

So, change your thinking on it because Rust is teaching you to be a better programmer.






























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
Rust is fantastic at ensuring that our code will work before I even go and run it. Rust makes you, forces you to get it right upfront. Most developers get really frustrated with the borrow checker and compiler whether or not it is going to compile. So, change your thinking on it because Rust is teaching you to be a better programmer. It`s teaching you about what is going to brake and why.

In C for example, you would not get this errors, everything would feel fine but you could end up writing a program that takes up all the memory and crashes a computer, and that is very easy to do in a language like C.

Rust will not make you do that and will help you understand, what is actualy happening with managing the memory, etc., in your program and doing things in the best way possible.

So, right now is comming up wiht an error **mismatched types**. 

That`s because he is expecting u32.




EVERYTHING BY DEFAULT IS IMMUTABLE. Everything is defaulted to immutable. If i wanted this to be a mutable variable, able to change the value of x, I would put in `let mut x = 50`

```rust
fn add_five(num: u32) -> u32 {

}

fn main() {
  let mut x: i32 = 50;
}
```

Its assuming x is an i32 wich is a signed intiger meaning it can go below 0, but we will force it to u32 because this is what our function expects.

```rust
fn add_five(num: u32) -> u32 {

}

fn main() {
  let mut x: u32 = 50;
  println!("{x}");
}
```

We need to close it with `;`, We are not returning printline so we put semicolon in to print the value of x.

We`ll now send x to the function add_five

```rust
fn add_five(num: u32) -> u32 {}

fn main() {
    let mut x: u32 = 50;
    println!("{x}");

    let y = add_five(x);
}
```

Complete the funtion and add `num + 5`

```rust
fn add_five(num: u32) -> u32 {
    num + 5
}

fn main() {
    let mut x: u32 = 50;
    println!("{x}");

    let y = add_five(x);
}
```


cargo test -- --nocapture

cargo test adds_five_test -- --nocapture

RUST_BACKTRACE=1 cargo test adds_five_test

cargo doc

cargo doc --open

cargo new --lib my_library

/// * `right` - a usize to add
///
/// # Example
///
/// ```
/// # use my_library::add; // Assuming the crate`s name is `my_library`
/// let l: usize = 20;
/// let r: usize = 5;
/// assert_eq!(add,(l, r), 25);
/// ```
```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

# 13. Dead Code and Unused Variables

```rust
#[allow(dead_code, unused_variables)]
fn _my_function (){
  let _x =
  let _y =
}

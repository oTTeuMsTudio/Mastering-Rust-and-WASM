mod my_funcs;

use crate::my_funcs::{add_five, subtract_10};

fn main() {
    let mut x: u32 = 50;
    println!("x is {x}");

    let y = add_five(x);
    println!("y is {y}");

    x = 70;
    println!("x is {x}");
}

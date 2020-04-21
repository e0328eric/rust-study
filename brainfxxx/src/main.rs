#![allow(unused)]
mod modint;
mod tape;

use crate::tape::Tape;
use crate::modint::ModInt;

fn main() {
    let tape = Tape::new();
    println!("{:?}", tape);
}

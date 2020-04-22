#![allow(unused)]
mod modint;
mod tape;

use crate::tape::Tape;
use crate::modint::ModInt;

fn main() {
    // Make a new Turing Tape
    let mut tape = Tape::new();

    // Add things on the right
    tape.add_right(ModInt(2));
    tape.add_right(ModInt(5));
    tape.add_right(ModInt(7));
    println!("{:?}", tape);

    // Add things on the left
    tape.add_left(ModInt(1));
    tape.add_left(ModInt(3));
    tape.add_left(ModInt(4));
    println!("{:?}", tape);

    // Move pointer to third position
    tape.move_right();
    tape.move_right();
    tape.move_right();
    println!("{:?}", tape);

    // Take a integer in that position
    println!("{:?}", tape.take_val());
}

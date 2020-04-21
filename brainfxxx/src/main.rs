use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Display, PartialEq)]
struct ModularInt(u8);

impl ModularInt {
    fn to_int(&self) -> u8 {
        self.0
    }
}

impl Add for ModularInt {
    type Output = ModularInt;
    fn add(self, rhs: Self) -> Self::Output {
        ModularInt(self.0.overflowing_add(rhs.0).0)
    }
}

fn main() {
    let x:u8 = 255;
    let y:u8 = 2;
    let z = x.overflowing_add(y).0;
    println!("{}", z);
}

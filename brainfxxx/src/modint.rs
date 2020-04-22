use std::ops::{Add, Sub};

// ModInt Type
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ModInt(pub u8);

impl ModInt {
    pub fn to_int(&self) -> u8 {
        self.0
    }
}

impl Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: Self) -> Self::Output {
        ModInt(self.0.overflowing_add(rhs.0).0)
    }
}

impl Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: Self) -> Self::Output {
        ModInt(self.0.overflowing_sub(rhs.0).0)
    }
}

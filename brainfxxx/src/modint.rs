use std::ops::{Add, Sub, Mul, Div};

// ModInt Type
#[derive(Debug, PartialEq)]
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

impl Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt(self.0.overflowing_mul(rhs.0).0)
    }
}

impl Div for ModInt {
    type Output = ModInt;
    fn div(self, rhs: Self) -> Self::Output {
        ModInt(self.0.overflowing_div(rhs.0).0)
    }
}

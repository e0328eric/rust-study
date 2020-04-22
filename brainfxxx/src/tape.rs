use std::collections::VecDeque;

use crate::modint::ModInt;

#[derive(Debug)]
pub struct Tape {
    tape: VecDeque<ModInt>,
    point: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            tape: VecDeque::new(),
            point: 0,
        }
    }

    pub fn add_right(&mut self, modint: ModInt) {
        self.tape.push_back(modint);
    }

    pub fn add_left(&mut self, modint: ModInt) {
        self.tape.push_front(modint);
    }

    pub fn move_right(&mut self) {
        if self.point == self.tape.len() - 1 {
            self.add_right(ModInt(0));
            self.point += 1;
        } else {
            self.point += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.point == 0 {
            self.add_left(ModInt(0));
        } else {
            self.point -= 1;
        }
    }

    pub fn take_val(&self) -> ModInt {
        self.tape[self.point]
    }

    pub fn change(&mut self, target: usize, change: ModInt) {
        self.tape[target] = change;
    }

    pub fn erase(&mut self, target: usize) {
        if target == self.tape.len() {
            self.tape.remove(target);
        } else {
            self.change(target, ModInt(0));
        }
    }
}

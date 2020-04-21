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

    pub fn extend_right(&mut self, modint: ModInt) {
        self.tape.push_back(modint);
    }

    pub fn extend_left(&mut self, modint: ModInt) {
        self.tape.push_front(modint);
    }

    pub fn erase(&mut self, target: usize) {
        if target == self.tape.len() {
            self.tape.remove(target);
        } else {
            self.tape.remove(target);
            self.tape.insert(target, ModInt(0));
        }
    }
}

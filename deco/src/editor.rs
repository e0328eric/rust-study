use crate::error;
use crate::Terminal;

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub struct WindowSize {
    pub row: usize,
    pub col: usize,
}

impl WindowSize {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

pub struct Editor {
    screen_size: WindowSize,
}

impl Editor {
    pub fn new() -> error::Result<Self> {
        Ok(Self {
            screen_size: Terminal::get_window_size()?,
        })
    }
}

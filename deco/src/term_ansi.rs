#![allow(unused)]
// This file stores all ANSI Escape code with constants

pub(crate) const ERASE_ENTIRE_SCREEN: &str = "\x1b[2J";
pub(crate) const MOVE_CURSOR_TOP_LEFT: &str = "\x1b[H";

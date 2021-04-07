use crate::util::Exp;
use crate::Position;
use crossterm::terminal;
use crossterm::{execute, ErrorKind};
use std::env;
use std::io::{stdout, Write};
use term::terminfo::TermInfo;
use unicode_width::UnicodeWidthStr;

pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Terminal {
    pub size: Size,
    regex: Exp,
}

impl Terminal {
    pub fn new() -> Result<Self, ErrorKind> {
        let size = terminal::size()?;
        Terminal::enter();
        Ok(Self {
            size: Size {
                width: size.0 as usize,
                height: size.1 as usize,
            },
            regex: Exp::new(),
        })
    }

    pub fn enter() {
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    }

    pub fn exit() {
        execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    pub fn goto(p: &Position) {
        execute!(stdout(), crossterm::cursor::MoveTo(p.x as u16, p.y as u16)).unwrap();
    }

    pub fn flush() {
        stdout().flush().unwrap();
    }

    pub fn hide_cursor() {
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
    }

    pub fn show_cursor() {
        execute!(stdout(), crossterm::cursor::Show).unwrap();
    }

    pub fn clear() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn align_break(&self, l: &str, r: &str) -> String {
        let left_length = UnicodeWidthStr::width(l);
        let right_length = UnicodeWidthStr::width(r);
        let padding = (self.size.width as usize).saturating_sub(left_length + right_length);
        " ".repeat(padding as usize)
    }

    pub fn align_left(&self, text: &str) -> String {
        let length = self.regex.ansi_len(text);
        let padding = (self.size.width as usize).saturating_sub(length);
        " ".repeat(padding as usize)
    }

    pub fn availability() -> usize {
        let color = env::var("COLORTERM");
        if color.unwrap_or_else(|_| "".to_string()) == "truecolor" {
            24
        } else if let Ok(info) = TermInfo::from_env() {
            if info.numbers.get("colors").unwrap() == &256 {
                256
            } else {
                16
            }
        } else {
            16
        }
    }
}

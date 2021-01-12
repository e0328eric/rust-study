use crate::map::GameMap;
use crate::Direction;
use crate::PlayerPos;
use crate::Terminal;
use std::io;

pub struct Render {
    pub map: GameMap,
    terminal: Terminal,
}

impl Render {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            map: GameMap::new("", 0, 0).0,
            terminal: Terminal::default()?,
        })
    }

    pub fn change_map(&mut self, source: &str, width: u16, height: u16) -> PlayerPos {
        let tmp = GameMap::new(source, width, height);
        self.map = tmp.0;
        tmp.1
    }

    pub fn refresh_screen(&self, game_start: bool, game_end: bool) -> io::Result<()> {
        if game_start {
            Terminal::hide_cursor();
            Terminal::cursor_position(0, 0);
        }
        if game_end {
            Terminal::clear_screen();
            Terminal::show_cursor();
        }
        self.draw_rows();
        Terminal::cursor_position(0, 0);
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let width = self.terminal.size().width;
        let height = self.terminal.size().height;
        let game_width = self.map.size().0;
        let game_height = self.map.size().1;
        let padding_width = width.saturating_sub(game_width * 2) / 2;
        let padding_height = height.saturating_sub(game_height) / 2;
        let padding = " ".repeat(padding_width as usize);
        let mut i = 0;
        for row in 0..=height {
            Terminal::clear_current_line();
            if padding_height <= row && row < padding_height.saturating_add(game_height) {
                print!("{}", padding);
                for j in i..i + game_width {
                    print!("{}", self.map.game_map().get(j as usize).unwrap());
                }
                i += game_width;
            }
            println!("\r");
        }
    }
}

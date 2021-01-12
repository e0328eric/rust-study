use crate::Render;
use crate::Terminal;
use std::io;
use termion::event::Key;

pub type PlayerPos = usize;

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    GameOver,
    StartGame,
    Running,
    NextLevel,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub struct Game {
    game_state: GameState,
    render: Render,
    player_pos: PlayerPos,
}

impl Game {
    pub fn new(source: &str, width: u16, height: u16) -> io::Result<Self> {
        let mut game = Self {
            game_state: GameState::StartGame,
            render: Render::new()?,
            player_pos: PlayerPos::default(),
        };
        game.player_pos = game.render.change_map(source, width, height);
        Ok(game)
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                panic!(err);
            }
            if self.game_state == GameState::GameOver {
                break;
            }
            if self.game_state == GameState::StartGame {
                self.game_state = GameState::Running;
            }
            if let Err(err) = self.process_keypress() {
                panic!(err);
            }
        }
    }

    fn refresh_screen(&self) -> io::Result<()> {
        self.render.refresh_screen(
            self.game_state == GameState::StartGame,
            self.game_state == GameState::GameOver,
        )
    }

    fn process_keypress(&mut self) -> io::Result<()> {
        let key = Terminal::read_key()?;
        match key {
            Key::Ctrl('c') | Key::Ctrl('d') => self.game_state = GameState::GameOver,
            Key::Right => self
                .render
                .player_move(&mut self.player_pos, Direction::Right),
            Key::Left => self
                .render
                .player_move(&mut self.player_pos, Direction::Left),
            Key::Up => self.render.player_move(&mut self.player_pos, Direction::Up),
            Key::Down => self
                .render
                .player_move(&mut self.player_pos, Direction::Down),
            _ => (),
        }
        Ok(())
    }
}

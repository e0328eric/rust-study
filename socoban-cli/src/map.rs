use crate::PlayerPos;
use std::fmt::{self, Display};
use termion::color;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MapBlockType {
    Wall,
    PlayerLocation,
    Rock,
    Goal,
    Blank,
}

impl Display for MapBlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapBlockType::Wall => write!(
                f,
                "{}  {}",
                color::Bg(color::White),
                color::Bg(color::Reset)
            ),
            MapBlockType::PlayerLocation => write!(
                f,
                "{}  {}",
                color::Bg(color::LightCyan),
                color::Bg(color::Reset)
            ),
            MapBlockType::Rock => write!(
                f,
                "{}  {}",
                color::Bg(color::LightBlue),
                color::Bg(color::Reset)
            ),
            MapBlockType::Goal => write!(
                f,
                "{}  {}",
                color::Bg(color::LightYellow),
                color::Bg(color::Reset)
            ),
            MapBlockType::Blank => write!(f, "{}  ", color::Bg(color::Reset)),
        }
    }
}

pub struct GameMap {
    width: u16,
    height: u16,
    map: Vec<MapBlockType>,
}

impl GameMap {
    pub fn new(source: &str, width: u16, height: u16) -> (Self, PlayerPos) {
        let mut map = Vec::new();
        let mut player_pos = PlayerPos::default();
        let mut find_player = false;

        for ch in source.chars() {
            match ch {
                '#' => map.push(MapBlockType::Wall),
                'P' => {
                    find_player = true;
                    map.push(MapBlockType::PlayerLocation);
                }
                'O' => map.push(MapBlockType::Rock),
                'G' => map.push(MapBlockType::Goal),
                '.' => map.push(MapBlockType::Blank),
                _ => continue,
            }
            if !find_player {
                player_pos += 1;
            }
        }

        (Self { width, height, map }, player_pos)
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn game_map(&self) -> &Vec<MapBlockType> {
        &self.map
    }

    pub fn game_map_mut(&mut self) -> &mut Vec<MapBlockType> {
        &mut self.map
    }

    // If the swap action is activated, return Some(()), else None
    pub fn swap(&mut self, a: usize, b: usize) -> Option<()> {
        if a < self.map.len() && b < self.map.len() {
            self.map.swap(a, b);
            return Some(());
        }
        None
    }
}

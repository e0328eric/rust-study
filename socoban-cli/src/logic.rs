use crate::map::MapBlockType;
use crate::Direction;
use crate::PlayerPos;
use crate::Render;

impl Render {
    pub fn player_move(&mut self, cur_pos: &mut PlayerPos, direction: Direction) {
        let width = self.map.size().0 as usize;
        match direction {
            Direction::Right => {
                let next_pos = if cur_pos.overflowing_add(1).1 {
                    return;
                } else {
                    cur_pos.overflowing_add(1).0
                };
                if !self.is_block_type(next_pos, MapBlockType::Wall) {
                    if self.map.swap(*cur_pos, next_pos).is_some() {
                        *cur_pos += 1;
                    }
                }
            }
            Direction::Left => {
                let next_pos = if cur_pos.overflowing_sub(1).1 {
                    return;
                } else {
                    cur_pos.overflowing_sub(1).0
                };
                if !self.is_block_type(next_pos, MapBlockType::Wall) {
                    if self.map.swap(*cur_pos, cur_pos.saturating_sub(1)).is_some() {
                        *cur_pos = cur_pos.saturating_sub(1);
                    }
                }
            }
            Direction::Up => {
                let next_pos = if cur_pos.overflowing_sub(width).1 {
                    return;
                } else {
                    cur_pos.overflowing_sub(width).0
                };
                if !self.is_block_type(next_pos, MapBlockType::Wall) {
                    if self
                        .map
                        .swap(*cur_pos, cur_pos.saturating_sub(width))
                        .is_some()
                    {
                        *cur_pos = cur_pos.saturating_sub(width);
                    }
                }
            }
            Direction::Down => {
                let next_pos = if cur_pos.overflowing_add(width).1 {
                    return;
                } else {
                    cur_pos.overflowing_add(width).0
                };
                if !self.is_block_type(next_pos, MapBlockType::Wall) {
                    if self.map.swap(*cur_pos, *cur_pos + width).is_some() {
                        *cur_pos += width;
                    }
                }
            }
        }
    }

    fn is_block_type(&self, nth: usize, expected: MapBlockType) -> bool {
        self.map.game_map().get(nth) == Some(&expected)
    }
}

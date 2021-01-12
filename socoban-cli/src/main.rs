mod game;
mod logic;
mod map;
mod render;
mod terminal;

pub use game::Direction;
pub use game::PlayerPos;
pub use render::Render;
pub use terminal::Terminal;

const TEST_MAP: &str = r#"
    ..................................
    #.................................
    #.................................
    ####..............................
    ....#.............................
    ....#............O...O............
    ....#.........................###.
    ....####.........O...O...........#
    ...............................P.#
    .................................#
    ..............................###.
"#;

fn main() {
    let mut game = game::Game::new(TEST_MAP, 34, 11).unwrap();
    game.run();
}

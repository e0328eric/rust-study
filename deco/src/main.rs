mod editor;
mod error;
mod term_ansi;
mod terminal;

use editor::Position;
use editor::WindowSize;
use terminal::Terminal;

fn main() -> error::Result<()> {
    let mut term = terminal::Terminal::new()?;
    let mut editor = editor::Editor::new()?;

    let mut is_quit = false;
    while !is_quit {
        term.refresh_screen();
        term.flush()?;
        term.process_keypress(&mut is_quit)?;
    }

    Ok(())
}

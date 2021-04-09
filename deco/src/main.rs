mod editor;
mod term_ansi;
mod terminal;

fn main() -> std::io::Result<()> {
    let mut term = terminal::Terminal::new();

    loop {
        term.refresh_screen();
        term.flush()?;
        term.process_keypress();
    }
}

extern crate piston;
extern crate glutin_window;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use glutin_window::GlutinWindow;

fn main() {
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");
    
    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(a) = events.next(&mut window) {

    }
}

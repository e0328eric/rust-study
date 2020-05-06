//! Gameboard controller.

use crate::gameboard::Gameboard;
use piston::input::GenericEvent;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController { gameboard }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {}
}

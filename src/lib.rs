//! Battleship game implemented in Rust

#![warn(missing_docs, clippy::unwrap_used)]

pub mod grid;
pub mod ship;

use crate::grid::Grid;
use crate::ship::Ship;
use crossterm::terminal::{Clear, ClearType};
use std::io::{self, Write};

/// Type alias for the standard [`Result`] type.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Runs the game.
pub fn run<W: Write>(mut output: W) -> Result<()> {
    // Prepare the game.
    let mut grid = Grid::new(10, 10);
    for _ in 0..3 {
        let ship = Ship::new_random(grid.width, grid.height);
        grid.place_ship(ship);
    }
    crossterm::execute!(output, Clear(ClearType::All))?;

    loop {
        // Show the game status.
        grid.display(&mut output)?;

        // Handle commands.
        let mut command = String::new();
        write!(output, "Command: ")?;
        output.flush()?;
        io::stdin().read_line(&mut command)?;

        // Clear the screen.
        crossterm::execute!(output, Clear(ClearType::FromCursorUp))?;
    }
}

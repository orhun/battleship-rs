//! Battleship game implemented in Rust

#![warn(missing_docs, clippy::unwrap_used)]

pub mod grid;

use crossterm::terminal::{Clear, ClearType};
use grid::Grid;
use std::io::{self, Write};

/// Type alias for the standard [`Result`] type.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Runs the game.
pub fn run<W: Write>(mut output: W) -> Result<()> {
    // Prepare the game.
    let grid = Grid::new(10, 10);
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

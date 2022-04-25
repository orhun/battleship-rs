//! Game grid.

use crate::ship::Ship;
use crate::Result;
use std::io::{Result as IoResult, Write};
use std::str;

/// Available characters for column names.
const ALPHABET_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

/// Representation of the game grid.
#[derive(Default, Debug)]
pub struct Grid {
    /// Width value.
    pub width: u8,
    /// Height value.
    pub height: u8,
    /// Ships on the grid.
    ships: Vec<Ship>,
}

impl Grid {
    /// Constructs a new instance of [`Grid`].
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            ships: Vec::new(),
        }
    }

    /// Constructs a new instance of [`Grid`] with random placement of ships.
    ///
    /// Also see [`Ship::new_random`].
    pub fn new_random(width: u8, height: u8) -> Self {
        let mut grid = Grid::new(width, height);
        for _ in 0..fastrand::i8(3..=5) {
            let ship = Ship::new_random(grid.width, grid.height);
            grid.place_ship(ship);
        }
        grid
    }

    /// Places a ship on the grid.
    pub fn place_ship(&mut self, ship: Ship) {
        // TODO: add rules for placing ships
        self.ships.push(ship);
    }

    /// Returns the grid as string.
    pub fn as_string(&self) -> Result<String> {
        let mut s = Vec::new();
        self.display(&mut s)?;
        Ok(str::from_utf8(&s)?.to_string())
    }

    /// Display a point on the grid.
    ///
    /// The point might be empty or a part of a ship.
    fn display_point<W: Write>(&self, out: &mut W, x: u8, y: u8) -> IoResult<()> {
        if let Some(ship) = self
            .ships
            .iter()
            .find(|ship| ship.coord.x == x && ship.coord.y == y)
        {
            write!(out, "{} ", if ship.hit != 0 { "☒" } else { "☐" })?;
        } else {
            write!(out, "• ")?;
        }
        Ok(())
    }

    /// Prints the grid to the given output.
    fn display<W: Write>(&self, out: &mut W) -> IoResult<()> {
        let alphabet_chars = ALPHABET_CHARS.chars().collect::<Vec<char>>();
        writeln!(out)?;
        for h in 0..self.height + 1 {
            if h == 0 {
                write!(out, "   ")?;
            } else if h == self.height {
                write!(out, "{} ", h)?;
            } else {
                write!(out, "{}  ", h)?;
            }
            for w in 0..self.width {
                if h == 0 {
                    write!(out, "{} ", alphabet_chars[w as usize].to_uppercase())?;
                } else {
                    self.display_point(out, w + 1, h)?;
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

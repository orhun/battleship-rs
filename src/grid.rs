//! Game grid.

use crate::ship::Ship;
use crate::Result;
use std::convert::TryFrom;
use std::io::{Result as IoResult, Write};
use std::result::Result as StdResult;
use std::str;

/// Available characters for column names.
const ALPHABET_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

/// Representation of coordinates on a 2-dimensional plane.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coordinate {
    /// X value.
    pub x: u8,
    /// Y value.
    pub y: u8,
}

impl<'a> TryFrom<String> for Coordinate {
    type Error = ();
    fn try_from(mut value: String) -> StdResult<Self, Self::Error> {
        value = value.to_lowercase();
        let mut coordinate = Coordinate::default();
        for (i, c) in ALPHABET_CHARS.chars().enumerate() {
            if value.starts_with(&c.to_string()) {
                value = value.trim_start_matches(c).to_string();
                coordinate.x = i as u8 + 1;
            }
        }
        if coordinate.x == 0 {
            Err(())
        } else if let Ok(y) = value.parse() {
            coordinate.y = y;
            Ok(coordinate)
        } else {
            Err(())
        }
    }
}

impl From<(u8, u8)> for Coordinate {
    fn from(v: (u8, u8)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

/// Representation of the game grid.
#[derive(Default, Debug)]
pub struct Grid {
    /// Width value.
    pub width: u8,
    /// Height value.
    pub height: u8,
    /// Ships on the grid.
    pub ships: Vec<Ship>,
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
    pub fn as_string(&self, show_ships: bool) -> Result<String> {
        let mut s = Vec::new();
        self.display(&mut s, show_ships)?;
        Ok(str::from_utf8(&s)?.to_string())
    }

    /// Display a point on the grid.
    ///
    /// The point might be empty or a part of a ship.
    fn display_point<W: Write>(&self, out: &mut W, x: u8, y: u8, show_ships: bool) -> IoResult<()> {
        if let Some(ship) = self
            .ships
            .iter()
            .find(|ship| ship.coord.x == x && ship.coord.y == y)
        {
            write!(
                out,
                "{} ",
                if ship.hit != 0 {
                    "☒"
                } else if show_ships {
                    "☐"
                } else {
                    "✕"
                }
            )?;
        } else {
            write!(out, "• ")?;
        }
        Ok(())
    }

    /// Prints the grid to the given output.
    fn display<W: Write>(&self, out: &mut W, show_ships: bool) -> IoResult<()> {
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
                    self.display_point(out, w + 1, h, show_ships)?;
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate() {
        let test_cases = [
            ((10, 10), "J10"),
            ((1, 7), "a7"),
            ((8, 5), "h5"),
            ((9, 1), "I1"),
            ((6, 8), "f8"),
            ((26, 2), "z2"),
        ];
        for (coord, coord_str) in test_cases {
            assert_eq!(
                Ok(Coordinate::from(coord)),
                Coordinate::try_from(coord_str.to_string()),
            );
        }
        assert!(Coordinate::try_from(String::from("test")).is_err());
        assert!(Coordinate::try_from(String::from("a999")).is_err());
        assert!(Coordinate::try_from(String::from("42")).is_err());
    }
}

//! Game board.

use crate::ship::{Ship, ShipType};
use crate::Result;
use std::convert::TryFrom;
use std::fmt;
use std::io::{Result as IoResult, Write};
use std::result::Result as StdResult;
use std::str;

/// Available alphabet characters for column names.
pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
/// The character that represents a hit.
const HIT_POINT: &str = "☒";
/// The character that represents a miss.
const MISSED_POINT: &str = "✕";
/// The character to display a default coordinate.
const DEFAULT_POINT: &str = "•";

/// Representation of coordinates on a 2-dimensional plane.
#[derive(Clone, Copy, Default)]
pub struct Coordinate {
    /// Point on the X axis.
    pub x: u8,
    /// Point on the Y axis.
    pub y: u8,
    /// Whether if the coordinate is hit.
    pub is_hit: bool,
}

/// Dismiss `is_hit` field during comparisons.
impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Omit `is_hit` field from debug output.
impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coordinate")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// For converting between e.g. (10, 10) to "J10"
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alphabet_chars = ALPHABET.chars().collect::<Vec<char>>();
        let index = self.x.checked_sub(1).unwrap_or_default() as usize;
        write!(f, "{}{}", alphabet_chars[index].to_uppercase(), self.y)
    }
}

/// For converting between e.g. "J10" to (10, 10)
impl TryFrom<String> for Coordinate {
    type Error = ();
    fn try_from(mut value: String) -> StdResult<Self, Self::Error> {
        value = value.to_lowercase();
        let mut coordinate = Coordinate::default();
        for (i, c) in ALPHABET.chars().enumerate() {
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

/// Support constructing a [`Coordinate`] from a tuple.
impl From<(u8, u8)> for Coordinate {
    fn from(v: (u8, u8)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            ..Self::default()
        }
    }
}

/// Representation of the game grid.
#[derive(Default, Debug)]
pub struct Grid {
    /// Width.
    pub width: u8,
    /// Height.
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

    /// Constructs a new instance of [`Grid`] with random ships and placements.
    ///
    /// Allows only one [`Battleship`] on the grid.
    /// Also see [`Ship::new_random`].
    ///
    /// [`Battleship`]: ShipType::Battleship
    pub fn new_random(width: u8, height: u8) -> Self {
        let mut grid = Grid::new(width, height);
        let ship_count = fastrand::usize(4..=7);
        let mut battleship = None;
        while grid.ships.len() != ship_count {
            let ship = Ship::new_random(grid.width, grid.height);
            if let ShipType::Battleship(_) = ship.type_ {
                battleship = Some(ship);
            } else {
                grid.place_ship(ship);
            }
        }
        if let Some(battleship) = battleship {
            grid.place_ship(battleship);
        }
        grid
    }

    /// Places a ship on the grid.
    ///
    /// Returns `false` if the ship is overlapping with other ships
    /// or placed outside the grid.
    /// Returns `true` if the placement is successful.
    pub fn place_ship(&mut self, ship: Ship) -> bool {
        let overlaps = self
            .ships
            .iter()
            .any(|s| s.coords.iter().any(|coord| ship.coords.contains(coord)));
        let overflows = ship
            .coords
            .iter()
            .any(|coord| coord.x > self.width || coord.y > self.height);
        if overlaps || overflows {
            false
        } else {
            self.ships.push(ship);
            true
        }
    }

    /// Returns the grid as string.
    ///
    /// Only hits/misses are shown if `show_ships` is true.
    pub fn as_string(&self, show_ships: bool) -> Result<String> {
        let mut s = Vec::new();
        self.display(&mut s, show_ships)?;
        Ok(str::from_utf8(&s)?.to_string())
    }

    /// Display a point on the grid.
    ///
    /// The point might be empty or a part of a ship.
    fn display_point<W: Write>(
        &self,
        out: &mut W,
        coordinate: Coordinate,
        show_ships: bool,
    ) -> IoResult<()> {
        if let Some(ship) = self
            .ships
            .iter()
            .find(|ship| ship.coords.contains(&coordinate))
        {
            write!(
                out,
                "{} ",
                if ship
                    .coords
                    .iter()
                    .find(|c| *c == &coordinate)
                    .map(|c| c.is_hit)
                    == Some(true)
                {
                    HIT_POINT.to_string()
                } else if show_ships {
                    ship.type_.to_string()
                } else {
                    MISSED_POINT.to_string()
                }
            )?;
        } else {
            write!(out, "{} ", DEFAULT_POINT)?;
        }
        Ok(())
    }

    /// Prints the grid to the given output.
    fn display<W: Write>(&self, out: &mut W, show_ships: bool) -> IoResult<()> {
        let alphabet_chars = ALPHABET.chars().collect::<Vec<char>>();
        writeln!(out)?;
        for h in 0..self.height + 1 {
            if h == 0 {
                write!(out, "   ")?;
            } else if h.to_string().len() == 2 {
                write!(out, "{} ", h)?;
            } else {
                write!(out, "{}  ", h)?;
            }
            for w in 0..self.width {
                if h == 0 {
                    write!(out, "{} ", alphabet_chars[w as usize].to_uppercase())?;
                } else {
                    self.display_point(out, Coordinate::from((w + 1, h)), show_ships)?;
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
    use crate::ship::Orientation;

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
            let coordinate = Coordinate::from(coord);
            assert_eq!(Ok(coordinate), Coordinate::try_from(coord_str.to_string()));
            assert_eq!(coord_str.to_uppercase(), coordinate.to_string())
        }
        assert!(Coordinate::try_from(String::from("test")).is_err());
        assert!(Coordinate::try_from(String::from("a999")).is_err());
        assert!(Coordinate::try_from(String::from("42")).is_err());
        assert_eq!(
            format!("{:?}", Coordinate::from((10, 2))),
            "Coordinate { x: 10, y: 2 }"
        );
    }

    #[test]
    fn test_grid() -> Result<()> {
        let mut grid = Grid::new(5, 5);
        assert!(grid.place_ship(Ship::new(ShipType::Boat, vec![Coordinate::from((2, 3))])));
        assert!(grid.place_ship(Ship::new(ShipType::Boat, vec![Coordinate::from((4, 5))])));
        assert!(grid.place_ship(Ship::new(
            ShipType::Destroyer(Orientation::Horizontal),
            vec![Coordinate::try_from(String::from("D1")).unwrap()]
        )));
        assert!(!grid.place_ship(Ship::new(
            ShipType::Battleship(Orientation::Vertical),
            ShipType::Battleship(Orientation::Vertical).get_hitbox(Coordinate::from((2, 4)))
        )));
        assert!(grid.place_ship(Ship::new(
            ShipType::Boat,
            vec![{
                let mut coordinate = Coordinate::try_from(String::from("B5")).unwrap();
                coordinate.is_hit = true;
                coordinate
            }]
        )));
        assert_eq!(
            r#"
   A B C D E 
1  • • • ▭ • 
2  • • • • • 
3  • △ • • • 
4  • • • • • 
5  • ☒ • △ • 
"#,
            grid.as_string(true)?
        );
        assert_eq!(
            r#"
   A B C D E 
1  • • • ✕ • 
2  • • • • • 
3  • ✕ • • • 
4  • • • • • 
5  • ☒ • ✕ • 
"#,
            grid.as_string(false)?
        );

        let grid = Grid::new_random(15, 15);
        assert!(!grid.ships.is_empty());
        Ok(())
    }
}

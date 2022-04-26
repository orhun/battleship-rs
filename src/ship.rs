//! A ship.

use crate::grid::Coordinate;
use std::fmt;

/// The character that represents a boat.
const BOAT: &str = "△";
/// The character that represents a part of a vertical destroyer ship.
const DESTROYER_VERTICAL: &str = "▯";
/// The character that represents a part of a horizontal destroyer ship.
const DESTROYER_HORIZONTAL: &str = "▭";
/// The character that represents a part of a battleship.
const BATTLESHIP: &str = "▧";

/// Available orientations for the ship.
#[derive(Copy, Clone, Debug)]
pub enum Orientation {
    /// Vertical placement.
    Vertical,
    /// Horizontal placement.
    Horizontal,
}

/// Available ship types.
#[derive(Copy, Clone, Debug)]
pub enum ShipType {
    /// 1x1 boat.
    Boat,
    /// 1x2 or 2x1 ship.
    Destroyer(Orientation),
    /// 3x2 or 2x3 ship.
    Battleship(Orientation),
}

impl Default for ShipType {
    fn default() -> Self {
        Self::Boat
    }
}

impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShipType::Boat => {
                    BOAT
                }
                ShipType::Destroyer(Orientation::Vertical) => {
                    DESTROYER_VERTICAL
                }
                ShipType::Destroyer(Orientation::Horizontal) => {
                    DESTROYER_HORIZONTAL
                }
                ShipType::Battleship(_) => {
                    BATTLESHIP
                }
            }
        )
    }
}

impl ShipType {
    /// Returns the possible variants for [`ShipType`].
    pub fn variants() -> Vec<Self> {
        vec![
            Self::Boat,
            Self::Destroyer(Orientation::Vertical),
            Self::Destroyer(Orientation::Horizontal),
            Self::Battleship(Orientation::Vertical),
            Self::Battleship(Orientation::Horizontal),
        ]
    }

    /// Returns a random [`ShipType`].
    pub fn new_random() -> Self {
        let types = Self::variants();
        types[fastrand::usize(..types.len())]
    }

    /// Returns the hit box of the ship.
    pub fn create_hitbox(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        match self {
            Self::Boat => {
                vec![coordinate]
            }
            Self::Destroyer(orientation) => {
                vec![coordinate, {
                    let mut coord = coordinate;
                    match orientation {
                        Orientation::Vertical => {
                            coord.y += 1;
                        }
                        Orientation::Horizontal => {
                            coord.x += 1;
                        }
                    }
                    coord
                }]
            }
            Self::Battleship(orientation) => {
                let mut coordinates = Vec::new();
                for i in 0..2 {
                    let mut coordinate = coordinate;
                    match orientation {
                        Orientation::Vertical => {
                            coordinate.x += i;
                        }
                        Orientation::Horizontal => {
                            coordinate.y += i;
                        }
                    }
                    for j in 0..3 {
                        coordinates.push({
                            let mut coordinate = coordinate;
                            match orientation {
                                Orientation::Vertical => {
                                    coordinate.y += j;
                                }
                                Orientation::Horizontal => {
                                    coordinate.x += j;
                                }
                            }
                            coordinate
                        });
                    }
                }
                coordinates
            }
        }
    }
}

/// Representation of a ship.
#[derive(Default, Debug)]
pub struct Ship {
    /// Ship type.
    pub type_: ShipType,
    /// Coordinates of the ship.
    pub coords: Vec<Coordinate>,
}

impl Ship {
    /// Constructs a new instance of [`Ship`].
    pub fn new(type_: ShipType, coords: Vec<Coordinate>) -> Self {
        Self { type_, coords }
    }

    /// Constructs a new instance of [`Ship`] with random properties.
    pub fn new_random(max_x: u8, max_y: u8) -> Self {
        let ship_type = ShipType::new_random();
        let coordinate = Coordinate::from((fastrand::u8(1..=max_x), fastrand::u8(1..=max_y)));
        Self::new(ship_type, ship_type.create_hitbox(coordinate))
    }

    /// Returns whether if the ship is sunk.
    pub fn is_sunk(&self) -> bool {
        self.coords.iter().all(|c| c.is_hit)
    }
}

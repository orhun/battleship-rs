//! A ship.

use crate::grid::Coordinate;

/// Available ship types.
#[derive(Copy, Clone, Debug)]
pub enum ShipType {
    /// 1x1 ship.
    Little = 1,
    // TODO: add more ships
}

impl Default for ShipType {
    fn default() -> Self {
        Self::Little
    }
}

impl ShipType {
    /// Returns the size of the ship.
    pub fn size(&self) -> u8 {
        *self as u8
    }
}

/// Representation of a ship.
#[derive(Default, Debug)]
pub struct Ship {
    /// Ship type.
    pub type_: ShipType,
    /// Coordinates of the ship.
    pub coord: Coordinate,
    /// Hit count.
    pub hit: u8,
}

impl Ship {
    /// Constructs a new instance of [`Ship`].
    pub fn new(type_: ShipType, coord: (u8, u8)) -> Self {
        Self {
            type_,
            coord: Coordinate::from(coord),
            hit: 0,
        }
    }

    /// Constructs a new instance of [`Ship`] with random properties.
    pub fn new_random(max_x: u8, max_y: u8) -> Self {
        Self::new(
            ShipType::Little,
            (fastrand::u8(1..=max_x), fastrand::u8(1..=max_y)),
        )
    }

    /// Returns whether if the ship is sunk.
    pub fn is_sunk(&self) -> bool {
        self.type_.size() == self.hit
    }
}

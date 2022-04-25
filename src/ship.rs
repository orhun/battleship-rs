//! A ship.

/// Available ship types.
#[derive(Copy, Clone, Debug)]
pub enum ShipType {
    /// 1x1 ship.
    Little = 1,
    // TODO: add more ships
}

impl ShipType {
    /// Returns the size of the ship.
    pub fn size(&self) -> u8 {
        *self as u8
    }
}

/// Representation of coordinates on a 2-dimensional plane.
#[derive(Debug)]
pub struct Coordinate {
    /// X value.
    pub x: u8,
    /// Y value.
    pub y: u8,
}

impl From<(u8, u8)> for Coordinate {
    fn from(v: (u8, u8)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

/// Representation of a ship.
#[derive(Debug)]
pub struct Ship {
    /// Ship type.
    type_: ShipType,
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
}

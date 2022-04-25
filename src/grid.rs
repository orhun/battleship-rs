//! Game grid.

use std::io::{Result, Write};

/// Available characters for column names.
const ALPHABET_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

/// Representation of the game grid.
#[derive(Debug)]
pub struct Grid {
    /// Width value.
    width: u8,
    /// Height value.
    height: u8,
}

impl Grid {
    /// Constructs a new instance of [`Grid`].
    pub fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    /// Prints the grid to the given output.
    pub fn display<W: Write>(&self, out: &mut W) -> Result<()> {
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
                    write!(out, "x ")?;
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

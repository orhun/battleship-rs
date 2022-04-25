//! Player.

use crate::grid::Coordinate;
use crate::grid::Grid;
use crate::{Result, BANNER};
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};

/// Representation of a player.
#[derive(Debug)]
pub struct Player {
    /// Name of the player.
    pub name: String,
    /// Player's grid.
    pub grid: Grid,
    /// Player's hits.
    pub hits: Vec<Coordinate>,
    /// TCP stream of the player.
    stream: TcpStream,
}

impl Player {
    /// Constructs a new instance of [`Player`].
    pub fn new(stream: TcpStream) -> Self {
        Self {
            name: String::new(),
            grid: Grid::default(),
            hits: Vec::new(),
            stream,
        }
    }

    /// Greets the player with a message and sets the name.
    pub fn greet(&mut self) -> Result<()> {
        self.send(&format!("{BANNER}\nPlease enter your name: "))?;
        self.name = self.read()?;
        Ok(())
    }

    /// Writes the given message to the TCP stream.
    pub fn send(&mut self, message: &str) -> Result<()> {
        Ok(self.stream.write_all(message.as_bytes())?)
    }

    /// Reads the next line from the TCP steram.
    pub fn read(&mut self) -> Result<String> {
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        Ok(line.trim().to_string())
    }

    /// Shuts down the TCP connection.
    pub fn exit(&mut self) -> Result<()> {
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        if let Ok(peer_addr) = self.stream.peer_addr() {
            println!("[+] Ending TCP connection with {:?}", peer_addr);
            if let Err(e) = self.exit() {
                eprintln!("[!] Failed to end TCP connection: {e}")
            }
        }
    }
}

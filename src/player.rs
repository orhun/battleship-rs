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
    /// TCP connection.
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
        self.send(&format!("{}\nPlease enter your name: ", BANNER))?;
        self.name = self.read()?;
        if self.name.is_empty() {
            self.name = String::from("unknown player");
        }
        Ok(())
    }

    /// Writes the given message to the TCP stream.
    pub fn send(&mut self, message: &str) -> Result<()> {
        Ok(self.stream.write_all(message.as_bytes())?)
    }

    /// Reads the next line from the TCP stream.
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

/// Shut down the TCP connection when the object goes out of scope.
impl Drop for Player {
    fn drop(&mut self) {
        if let Ok(peer_addr) = self.stream.peer_addr() {
            println!("[+] Ending TCP connection with {:?}", peer_addr);
            if let Err(e) = self.exit() {
                eprintln!("[!] Failed to end TCP connection: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    #[ignore]
    fn test_player() -> Result<()> {
        let address = "0.0.0.0:3333";
        let listener = TcpListener::bind(address)?;
        thread::spawn(move || listener.accept());
        let stream = TcpStream::connect(address)?;
        let mut player = Player::new(stream);
        player.greet()?;
        assert_eq!("unknown player", player.name);
        Ok(())
    }
}

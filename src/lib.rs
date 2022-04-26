//! Battleship game implemented in Rust

#![warn(missing_docs, clippy::unwrap_used)]

pub mod game;
pub mod grid;
pub mod player;
pub mod ship;

use crate::game::Game;
use crate::grid::{Grid, ALPHABET};
use crate::player::Player;
use std::io::{Error as IoError, ErrorKind};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

/// ASCII art for the banner.
const BANNER: &str = r#"        _    _
     __|_|__|_|__
   _|____________|__
  |o o o o o o o o /
~'`~'`~'`~'`~'`~'`~'`~
Welcome to Battleship!"#;

/// Type alias for the standard [`Result`] type.
///
/// See <https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html>
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Runs the game.
pub fn run(socket_addr: &str, grid_width: u8, grid_height: u8) -> Result<()> {
    // Prepare the game.
    if usize::from(grid_width) > ALPHABET.len() || usize::from(grid_height) > ALPHABET.len() {
        return Err("[!] Invalid grid dimensions.".into());
    }
    let game = Arc::new(Mutex::new(Game::default()));

    // Start listening for connections.
    let listener = TcpListener::bind(socket_addr)?;
    println!("[+] Server is listening on {}", socket_addr);

    // Handle connections.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[+] New connection: {}", stream.peer_addr()?);
                let mut player = Player::new(stream);
                if game.try_lock().is_err() {
                    player.send("Lobby is full. Please wait.\n")?;
                    continue;
                }
                let game = Arc::clone(&game);
                thread::spawn(move || {
                    // Add a player to the game.
                    let add_new_player = || -> Result<()> {
                        player.greet()?;
                        let mut game = game.lock().expect("failed to retrieve game");
                        game.add_player(player)?;
                        // Start the game when ready.
                        if game.is_ready() {
                            // Assign random boards to the players.
                            for player in game.players.iter_mut() {
                                player.grid = Grid::new_random(grid_width, grid_height);
                                println!(
                                    "[#] {}'s grid:{}",
                                    player.name,
                                    player.grid.as_string(true)?
                                );
                            }
                            // Start the game loop.
                            game.start(grid_width, grid_height)?;
                        }
                        Ok(())
                    };

                    // Handle errors.
                    if let Err(e) = add_new_player() {
                        eprintln!("[!] Gameplay error: {}", e);
                        if let Ok(io_error) = e.downcast::<IoError>() {
                            if io_error.kind() == ErrorKind::BrokenPipe {
                                let mut game = game.lock().expect("failed to retrieve game");
                                game.players.iter_mut().for_each(|player| {
                                    let _ = player.send("Your opponent left the game.\n");
                                });
                                game.players.clear();
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("[!] Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

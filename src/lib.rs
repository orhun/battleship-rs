//! Battleship game implemented in Rust

#![warn(missing_docs, clippy::unwrap_used)]

pub mod game;
pub mod grid;
pub mod player;
pub mod ship;

use crate::game::Game;
use crate::grid::Grid;
use crate::player::Player;
use std::io::{Error as IoError, ErrorKind};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

/// Type alias for the standard [`Result`] type.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Runs the game.
pub fn run(socket_addr: &str) -> Result<()> {
    let listener = TcpListener::bind(socket_addr)?;
    println!("[+] Server is listening on {}", socket_addr);
    let (grid_width, grid_height) = (10, 10);
    let game = Arc::new(Mutex::new(Game::default()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[+] New connection: {}", stream.peer_addr()?);
                let mut player = Player::new(stream);
                if game.try_lock().is_err() {
                    player.send("Lobby is full.\n")?;
                    continue;
                }
                let game = Arc::clone(&game);
                thread::spawn(move || {
                    let mut game = game.lock().expect("failed to retrieve game");
                    let start_game = || -> Result<()> {
                        player.greet()?;
                        game.add_player(player)?;
                        if game.is_ready() {
                            for player in game.players.iter_mut() {
                                player.grid = Grid::new_random(grid_width, grid_height);
                                println!(
                                    "[#] {}'s grid:{}",
                                    player.name,
                                    player.grid.as_string(true)?
                                );
                            }
                            game.play(grid_width, grid_height)?;
                        }
                        Ok(())
                    };
                    if let Err(e) = start_game() {
                        eprintln!("[!] Gameplay error: {}", e);
                        if let Ok(io_error) = e.downcast::<IoError>() {
                            if io_error.kind() == ErrorKind::BrokenPipe {
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

//! Main game.

use crate::grid::Coordinate;
use crate::grid::Grid;
use crate::player::Player;
use crate::ship::Ship;
use crate::Result;
use std::convert::TryFrom;
use std::thread;
use std::time::Duration;

/// Maximum number of players.
const MAX_PLAYERS: usize = 2;

/// Representation of the Battleship game.
#[derive(Default, Debug)]
pub struct Game {
    /// Players of the game.
    pub players: Vec<Player>,
}

impl Game {
    /// Checks if the players are connected.
    pub fn is_ready(&self) -> bool {
        self.players.len() == MAX_PLAYERS
    }

    /// Adds a new player to the game.
    pub fn add_player(&mut self, player: Player) -> Result<()> {
        if self.players.get(0).is_none() {
            self.players.push(player);
            self.players[0].send("Waiting for opponent...\n")?;
        } else {
            self.players.push(player);
            for i in 0..MAX_PLAYERS {
                let message = format!(
                    "Your opponent is {}\n",
                    self.players[MAX_PLAYERS - (i + 1)].name
                );
                self.players[i].send(&message)?;
            }
        }
        Ok(())
    }

    /// Shows the countdown for starting the game.
    fn show_countdown(&mut self) -> Result<()> {
        println!("[#] Game is starting.");
        for i in 1..4 {
            let message = format!("Game starts in {}...\n", 4 - i);
            self.players.iter_mut().try_for_each(|p| p.send(&message))?;
            thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    /// Shows the grid of the players.
    fn show_grid(&mut self, width: u8, height: u8) -> Result<()> {
        for i in 0..MAX_PLAYERS {
            let ships = self.players[i]
                .hits
                .iter()
                .map(|coord| Ship {
                    coords: vec![Coordinate {
                        x: coord.x,
                        y: coord.y,
                        is_hit: self.players[MAX_PLAYERS - (i + 1)]
                            .grid
                            .ships
                            .iter()
                            .any(|ship| ship.coords.contains(coord)),
                    }],
                    ..Default::default()
                })
                .collect();
            let grid_str = Grid {
                width,
                height,
                ships,
            }
            .as_string(false)?;
            self.players[i].send(&grid_str)?;
            let grid_str = self.players[i].grid.as_string(true)?;
            self.players[i].send(&grid_str)?;
        }
        Ok(())
    }

    /// Starts the game.
    /// TODO: add a more descriptive comment.
    pub fn play(&mut self, grid_width: u8, grid_height: u8) -> Result<()> {
        self.show_countdown()?;
        'game: loop {
            for i in 0..MAX_PLAYERS {
                if self.players[i].grid.ships.iter().all(|ship| ship.is_sunk()) {
                    let message = format!("{} won.\n", self.players[MAX_PLAYERS - (i + 1)].name);
                    self.players[i].send(&message)?;
                    self.players[MAX_PLAYERS - (i + 1)].send("You won!\n")?;
                    self.players.clear();
                    print!("[#] {}", message);
                    break 'game;
                }

                self.show_grid(grid_width, grid_height)?;

                self.players[i].send("Your turn: ")?;
                let message = format!("{}'s turn.\n", self.players[i].name);
                print!("[#] {}", message);
                self.players[MAX_PLAYERS - (i + 1)].send(&message)?;
                let coordinate_str = self.players[i].read()?;
                let coordinate =
                    if let Ok(coordinate) = Coordinate::try_from(coordinate_str.to_string()) {
                        println!(
                            "[#] {} is firing a shot: {} ({:?})",
                            self.players[i].name, coordinate_str, coordinate
                        );
                        coordinate
                    } else {
                        self.players[i].send("Your missile went to space!\n")?;
                        continue;
                    };
                self.players[i].hits.push(coordinate);
                if let Some(coordinate) = self.players[MAX_PLAYERS - (i + 1)]
                    .grid
                    .ships
                    .iter_mut()
                    .find(|ship| ship.coords.contains(&coordinate))
                    .and_then(|ship| ship.coords.iter_mut().find(|c| *c == &coordinate))
                {
                    coordinate.is_hit = true;
                    self.players[i].send("Hit!\n")?;
                } else {
                    self.players[i].send("Missed.\n")?;
                }
            }
        }
        Ok(())
    }
}

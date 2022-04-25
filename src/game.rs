//! Main game.

use crate::grid::Grid;
use crate::player::Player;
use crate::Result;
use std::thread;
use std::time::Duration;

/// Maximum number of players.
const MAX_PLAYERS: usize = 2;

/// Representation of the Battleship game.
#[derive(Default, Debug)]
pub struct Game {
    /// Players of the game.
    players: Vec<Player>,
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
            self.players[0].send_message("Waiting for opponent...\n")?;
        } else {
            self.players.push(player);
            for i in 0..self.players.len() {
                let message = format!(
                    "Your opponent is {}\n",
                    self.players[self.players.len() - (i + 1)].name
                );
                self.players[i].send_message(&message)?;
            }
        }
        Ok(())
    }

    /// Starts the game.
    /// TODO: add a more descriptive comment
    pub fn play(&mut self, grid_width: u8, grid_height: u8) -> Result<()> {
        for player in self.players.iter_mut() {
            player.grid = Grid::new_random(grid_width, grid_height);
        }
        for i in 1..4 {
            let message = format!("Game starts in {}...\n", 4 - i);
            self.players
                .iter_mut()
                .try_for_each(|p| p.send_message(&message))?;
            thread::sleep(Duration::from_secs(1));
        }

        for i in 0..self.players.len() {
            let grid_str = self.players[self.players.len() - (i + 1)]
                .grid
                .as_string()?;
            self.players[i].send_message(&grid_str)?;
            let grid_str = self.players[i].grid.as_string()?;
            self.players[i].send_message(&grid_str)?;
        }

        Ok(())
    }
}

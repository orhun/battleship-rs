use std::env;
use std::process;

/// Default TCP socket address to serve the game.
const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:1234";
/// Environment variable for setting the TCP socket address.
const SOCKET_ADDR_ENV: &str = concat!(env!("CARGO_PKG_NAME"), "_socket");
/// Environment variable for setting the grid width.
const GRID_WIDTH_ENV: &str = concat!(env!("CARGO_PKG_NAME"), "_grid_width");
/// Environment variable for setting the grid height.
const GRID_HEIGHT_ENV: &str = concat!(env!("CARGO_PKG_NAME"), "_grid_height");

fn main() {
    // Get the TCP address.
    let socket_addr = env::var(SOCKET_ADDR_ENV.to_uppercase())
        .unwrap_or_else(|_| DEFAULT_SOCKET_ADDR.to_string());

    // Get the game configuration.
    let (grid_width, grid_height) = (
        env::var(GRID_WIDTH_ENV.to_uppercase())
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(10),
        env::var(GRID_HEIGHT_ENV.to_uppercase())
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(10),
    );

    // Run the game.
    match battleship::run(&socket_addr, grid_width, grid_height) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

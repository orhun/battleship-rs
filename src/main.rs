use std::env;
use std::process;

/// Default TCP socket address to serve the game.
const DEFAULT_SOCKET_ADDR: &str = "0.0.0.0:1234";

/// Environment variable for setting the TCP socket address.
const SOCKET_ADDR_ENV: &str = concat!(env!("CARGO_PKG_NAME"), "_SOCKET");

fn main() {
    // Get the TCP address.
    let socket_addr = env::var(SOCKET_ADDR_ENV.to_uppercase())
        .unwrap_or_else(|_| DEFAULT_SOCKET_ADDR.to_string());

    // Run the game.
    match battleship::run(&socket_addr) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

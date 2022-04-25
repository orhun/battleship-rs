use std::env;
use std::process;

fn main() {
    // Set the default logging level.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    // Initialize the logger.
    pretty_env_logger::init();

    // Run the game.
    match battleship::run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

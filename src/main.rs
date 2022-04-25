use std::process;

fn main() {
    // Run the game.
    match battleship::run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

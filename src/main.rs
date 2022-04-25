use std::io;
use std::process;

fn main() {
    let stdout = io::stdout();
    match battleship::run(stdout) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

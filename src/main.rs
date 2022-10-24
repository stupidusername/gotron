use std::process;

use gotron;

fn main() {
    if let Err(e) = gotron::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

mod commands;
mod tasks;

use crate::commands::{display_args, match_input_to_command};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    display_args(&args);
    match match_input_to_command(&args) {
        Ok(commande) => commande.execute(),
        Err(err) => {
            println!("\nUnknown command: {}\n", err.input)
        }
    }
}

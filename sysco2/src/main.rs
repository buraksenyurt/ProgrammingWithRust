mod command_error;
mod utility;

use crate::command_error::CommandError;
use crate::utility::Utility;
use std::{env, process};

fn main() -> Result<(), CommandError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Utility::help();
        process::exit(1);
    }

    if let Err(e) = Utility::run(&args[1]) {
        eprintln!("{}", e);
        process::exit(1);
    };

    Ok(())
}

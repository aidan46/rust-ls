mod cli;
mod error;

use clap::Parser;
use cli::Cli;
use error::Error;
use std::{path::Path, process::exit};

fn main() {
    let args = Cli::parse();
    let path_str = match args.path {
        Some(p) => p,
        None => String::from("."),
    };
    let path = Path::new(&path_str);
    if !path.exists() {
        eprintln!("{}", Error::PathNotFound(path_str));
        exit(1);
    }
}

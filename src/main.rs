mod cli;
mod content;
mod error;

use clap::Parser;
use cli::Cli;
use content::Content;
use error::Error;
use std::{path::PathBuf, process::exit};

fn main() {
    let args = Cli::parse();
    let files = args.file;
    for file in files {
        let path = PathBuf::from(&file);
        if !path.exists() {
            eprintln!("{}", Error::PathNotFound(file));
            exit(1);
        }
        let content = Content::from_path(&path);

        println!("{:#?}", content);
    }
}

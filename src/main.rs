mod cli;
mod content;
mod error;

use clap::Parser;
use cli::Cli;
use content::Content;
use error::Error;
use std::{path::PathBuf, process::exit};

#[derive(Debug)]
struct RustLs {
    cli: Cli,
    paths: Vec<String>,
    contents: Vec<Content>,
}

impl RustLs {
    fn new(cli: Cli) -> Self {
        Self {
            cli,
            paths: vec![],
            contents: vec![],
        }
    }

    fn add_path(&mut self, path: String, content: Content) {
        self.paths.push(path);
        self.contents.push(content);
    }

    fn print_output(&self, sorting: Sorting) {
        let cli = &self.cli;
        if cli.recursive {
            // Recursive printing
            for content in &self.contents {
                content.print_recurse(cli.all, sorting);
            }
        } else {
            // Regular printing
            if !cli.long {
                for content in &self.contents {
                    content.print_short(cli.all, sorting);
                }
            } else {
                for content in &self.contents {
                    content.print_long(cli.all);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Sorting {
    Normal,
    Time,
    Reverse,
    TimeReverse,
}

fn main() {
    let cli = Cli::parse();
    let sorting = match (cli.timesort, cli.reverse) {
        (false, false) => Sorting::Normal,
        (true, false) => Sorting::Time,
        (false, true) => Sorting::Reverse,
        (true, true) => Sorting::TimeReverse,
    };
    let files = cli.files.clone();
    let mut rust_ls = RustLs::new(cli);
    for file in files {
        let path = PathBuf::from(&file);
        if !path.exists() {
            eprintln!("{}", Error::PathNotFound(file));
            exit(1);
        }
        rust_ls.add_path(file, Content::from_path(&path));
    }

    rust_ls.print_output(sorting);
}

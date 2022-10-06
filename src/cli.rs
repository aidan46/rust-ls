use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[clap(value_parser, default_value = ".")]
    pub(crate) file: Vec<String>,
    #[clap(short = 'a', long, action)]
    /// Do not ignore entries starting with .
    all: bool,
    #[clap(short = 'l', long, action)]
    /// Display extened file metadata as a table
    long: bool,
    #[clap(short = 'r', long, action)]
    /// Reverse the order of the sort
    reverse: bool,
    #[clap(short = 'R', long, action)]
    /// Recurse into directories
    recursive: bool,
    #[clap(short = 't', long, action)]
    /// Sort by time modified
    timesort: bool,
}

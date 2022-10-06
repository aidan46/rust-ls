use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[clap(value_parser, default_value = ".")]
    pub(crate) files: Vec<String>,
    #[clap(short = 'a', long, action)]
    /// Do not ignore entries starting with .
    pub(crate) all: bool,
    #[clap(short = 'l', long, action)]
    /// Display extened file metadata as a table
    pub(crate) long: bool,
    #[clap(short = 'r', long, action)]
    /// Reverse the order of the sort
    pub(crate) reverse: bool,
    #[clap(short = 'R', long, action)]
    /// Recurse into directories
    pub(crate) recursive: bool,
    #[clap(short = 't', long, action)]
    /// Sort by time modified
    pub(crate) timesort: bool,
}

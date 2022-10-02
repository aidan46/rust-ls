use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[clap(value_parser, default_value = ".")]
    pub(crate) file: Vec<String>,
    #[clap(short = 'a', long, action)]
    all: bool,
    #[clap(short = 'l', long, action)]
    list: bool,
    #[clap(short = 'r', long, action)]
    reverse: bool,
    #[clap(short = 'R', long, action)]
    recursive: bool,
    #[clap(short = 't', action)]
    timesort: bool,
}

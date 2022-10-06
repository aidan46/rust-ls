# rust-ls
Oops, I rewrote `ls` in Rust.

## Basic usage
```bash
rust-ls

USAGE:
    rust-ls [OPTIONS] [FILE]...

ARGS:
    <FILE>...    [default: .]

OPTIONS:
    -a, --all          Do not ignore entries starting with
    -h, --help         Print help information
    -l, --long         Display extened file metadata as a table
    -r, --reverse      Reverse the order of the sort
    -R, --recursive    Recurse into directories
    -t, --timesort     Sort by time modified
```

## Command-line options
- [x] Basic usage
- [x] -a, --all, do not ignore entries starting with
- [ ] -l, --long, display extened file metadata as a table
- [x] -r, --reverse, reverse the order of the sort
- [ ] -R, --recursive, recurse into directories
- [ ] -t, --timesort, sort by time modified

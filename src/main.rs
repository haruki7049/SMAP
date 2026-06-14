use clap::Parser;
use smap::cli::CLIArgs;

fn main() {
    let args: CLIArgs = CLIArgs::parse();
    dbg!(args);
}

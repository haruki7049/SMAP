use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct CLIArgs {
    #[arg(short, long)]
    target: SocketAddr,

    file: PathBuf,
}

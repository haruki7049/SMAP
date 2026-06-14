use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct CLIArgs {
    #[arg(short, long)]
    target: SocketAddr,
}

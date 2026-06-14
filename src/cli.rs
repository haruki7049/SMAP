use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct CLIArgs {
    #[args(short, long)]
    target: SocketAddr,
}

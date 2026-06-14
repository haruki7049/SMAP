use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct CLIArgs {
    /// A target IP.
    /// If this is none, play back via your local device.
    #[arg(short, long)]
    pub target: Option<SocketAddr>,

    /// An audio file path
    pub filepath: PathBuf,
}

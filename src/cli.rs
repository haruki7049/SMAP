use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;

use crate::players::Volume;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct CLIArgs {
    /// A target IP.
    /// If this is none, play back via your local device.
    #[arg(short, long)]
    pub target: Option<SocketAddr>,

    /// An audio file path
    pub filepath: PathBuf,

    /// Play back volume
    #[arg(short, long, default_value_t = 1.0)]
    pub volume: Volume,
}

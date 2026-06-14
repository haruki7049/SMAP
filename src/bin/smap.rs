use clap::Parser;
use smap::cli::CLIArgs;
use smap::players::AudioPlayer;
use smap::players::local::LocalAudioPlayer;
use smap::players::remote::RemoteAudioPlayer;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    dbg!(&args);

    match &args.target {
        None => local_play(&args)?,
        Some(socket) => remote_play(&args, socket)?,
    }

    Ok(())
}

fn local_play(args: &CLIArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut local_player = LocalAudioPlayer::default();
    local_player.volume(args.volume);
    local_player.play(&args.filepath)?;

    Ok(())
}

fn remote_play(args: &CLIArgs, socket: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let mut remote_player = RemoteAudioPlayer::new(args.volume, *socket);
    remote_player.volume(args.volume);
    remote_player.play(&args.filepath)?;

    todo!()
}

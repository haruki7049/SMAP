use clap::Parser;
use smap::{
    cli::CLIArgs,
    players::{AudioPlayer, local::LocalAudioPlayer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    dbg!(&args);

    if let Some(target) = args.target {
        todo!("The remote play feature has not been implemented yet, args.target: {target}");
    }

    let mut local_player = LocalAudioPlayer::default();
    local_player.volume(args.volume);
    local_player.play(&args.filepath)?;

    Ok(())
}

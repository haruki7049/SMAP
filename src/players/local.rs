use std::{fs::File, io::BufReader};

use super::AudioPlayer;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct LocalAudioPlayer {}

#[derive(Debug, Error)]
pub enum LocalAudioPlayerError {
    #[error("Device sink error via rodio crate: {0:?}")]
    RodioDeviceSink(#[from] rodio::DeviceSinkError),

    #[error("Play error via rodio crate: {0:?}")]
    RodioPlayError(#[from] rodio::stream::PlayError),

    #[error("IO error via std crate: {0:?}")]
    Io(#[from] std::io::Error),
}

impl AudioPlayer for LocalAudioPlayer {
    type Error = LocalAudioPlayerError;

    fn play(&self, filepath: &std::path::PathBuf) -> Result<(), Self::Error> {
        let sink_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        let reader = BufReader::new(File::open(filepath)?);
        let player = rodio::play(&sink_handle.mixer(), reader)?;
        player.sleep_until_end();

        Ok(())
    }
}

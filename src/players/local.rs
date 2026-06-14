use super::AudioPlayer;
use super::Volume;
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct LocalAudioPlayer {
    volume: Volume,
}

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
        let mut sink_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        sink_handle.log_on_drop(false); // Disable log via sink_handle
        let reader = BufReader::new(File::open(filepath)?);
        let player = rodio::play(&sink_handle.mixer(), reader)?;
        player.set_volume(self.volume);
        player.sleep_until_end();

        Ok(())
    }

    fn volume(&mut self, volume: Volume) {
        self.volume = volume;
    }
}

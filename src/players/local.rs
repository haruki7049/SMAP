use super::AudioPlayer;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct LocalAudioPlayer {}

#[derive(Debug, Error)]
pub enum LocalAudioPlayerError {}

impl AudioPlayer for LocalAudioPlayer {
    type Error = LocalAudioPlayerError;

    fn play(&self, filepath: &std::path::PathBuf) -> Result<(), Self::Error> {
        todo!()
    }
}

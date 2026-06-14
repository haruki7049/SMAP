use std::net::SocketAddr;
use std::path::Path;

use super::AudioPlayer;
use super::Volume;
use thiserror::Error;

#[derive(Debug)]
pub struct RemoteAudioPlayer {
    volume: Volume,
    socket: SocketAddr,
}

impl RemoteAudioPlayer {
    pub fn new(volume: Volume, socket: SocketAddr) -> Self {
        Self { volume, socket }
    }
}

#[derive(Debug, Error)]
pub enum RemoteAudioPlayerError {}

impl AudioPlayer for RemoteAudioPlayer {
    type Error = RemoteAudioPlayerError;

    fn play(&self, filepath: &Path) -> Result<(), Self::Error> {
        todo!()
    }

    fn volume(&mut self, volume: Volume) {
        self.volume = volume;
    }
}

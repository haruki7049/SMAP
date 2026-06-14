use std::path::Path;

pub mod local;
pub mod remote;

pub trait AudioPlayer {
    type Error;

    fn play(&self, filepath: &Path) -> Result<(), Self::Error>;
    fn volume(&mut self, volume: Volume);
}

pub type Volume = f32;

use std::path::PathBuf;

pub mod local;

pub trait AudioPlayer {
    type Error;

    fn play(&self, filepath: &PathBuf) -> Result<(), Self::Error>;
}

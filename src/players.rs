use serde::Deserialize;
use serde::Serialize;
use std::net::SocketAddr;
use std::path::Path;

pub mod local;
pub mod remote;

pub trait AudioPlayer {
    type Error;

    fn play(&self, filepath: &Path) -> Result<(), Self::Error>;
    fn volume(&mut self, volume: Volume);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
    pub volume: Volume,
    pub socket: SocketAddr,
}

impl Packet {
    pub fn new(bytes: Vec<u8>, volume: Volume, socket: SocketAddr) -> Self {
        Self {
            bytes,
            volume,
            socket,
        }
    }
}

pub type Volume = f32;

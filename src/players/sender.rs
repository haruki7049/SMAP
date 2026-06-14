use super::AudioPlayer;
use super::Packet;
use super::Volume;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::path::Path;
use thiserror::Error;

#[derive(Debug)]
pub struct Sender {
    volume: Volume,
    socket: SocketAddr,
}

impl Sender {
    pub fn new(volume: Volume, socket: SocketAddr) -> Self {
        Self { volume, socket }
    }
}

#[derive(Debug, Error)]
pub enum SenderError {
    #[error("IO error from std crate: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serde encoding error from serde_json crate: {0}")]
    SerdeEncoding(#[from] serde_json::Error),
}

impl AudioPlayer for Sender {
    type Error = SenderError;

    fn play(&self, filepath: &Path) -> Result<(), Self::Error> {
        let volume = self.volume;
        let socket = self.socket;

        let mut bytes: Vec<u8> = Vec::new();
        File::open(filepath)?.read_to_end(&mut bytes)?;
        let packet = Packet::new(bytes, volume, socket);

        // Send packet struct to remote PC via TCP
        let v: Vec<u8> = serde_json::to_vec(&packet)?;
        let mut stream: TcpStream = TcpStream::connect(&socket)?;
        stream.write_all(&v)?;

        Ok(())
    }

    fn volume(&mut self, volume: Volume) {
        self.volume = volume;
    }
}

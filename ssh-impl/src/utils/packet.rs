use std::io::{Read, Write};
use anyhow::{Result, Context};

/// SSH packet structure (simplified)
/// Format: [packet_length(4)][padding_length(1)][payload][padding][mac]
pub struct Packet {
    pub payload: Vec<u8>,
}

impl Packet {
    /// Create a new packet with payload
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }

    /// Read a packet from a reader (unencrypted, for initial phases)
    pub fn read(reader: &mut dyn Read) -> Result<Self> {
        // Read packet length (4 bytes, big-endian)
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)
            .context("Failed to read packet length")?;
        let packet_length = u32::from_be_bytes(len_buf) as usize;

        // Read padding length (1 byte)
        let mut pad_len_buf = [0u8; 1];
        reader.read_exact(&mut pad_len_buf)
            .context("Failed to read padding length")?;
        let padding_length = pad_len_buf[0] as usize;

        // Read payload
        let payload_length = packet_length - padding_length - 1;
        let mut payload = vec![0u8; payload_length];
        reader.read_exact(&mut payload)
            .context("Failed to read payload")?;

        // Read and discard padding
        let mut padding = vec![0u8; padding_length];
        reader.read_exact(&mut padding)
            .context("Failed to read padding")?;

        Ok(Self { payload })
    }

    /// Write a packet to a writer (unencrypted, for initial phases)
    pub fn write(&self, writer: &mut dyn Write) -> Result<()> {
        // Calculate padding length (4-8 bytes for simplicity)
        let padding_length = 8 - (self.payload.len() % 8);
        let padding: Vec<u8> = vec![0u8; padding_length];
        
        let packet_length = (self.payload.len() + padding_length + 1) as u32;
        
        // Write packet length
        writer.write_all(&packet_length.to_be_bytes())
            .context("Failed to write packet length")?;
        
        // Write padding length
        writer.write_all(&[padding_length as u8])
            .context("Failed to write padding length")?;
        
        // Write payload
        writer.write_all(&self.payload)
            .context("Failed to write payload")?;
        
        // Write padding
        writer.write_all(&padding)
            .context("Failed to write padding")?;
        
        writer.flush()
            .context("Failed to flush writer")?;
        
        Ok(())
    }
}


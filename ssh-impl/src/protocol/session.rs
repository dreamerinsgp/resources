use anyhow::{Result, Context};
use crate::crypto::dh::SessionKeys;
use crate::crypto::encryption::EncryptionContext;
use crate::utils::packet::Packet;
use crate::utils::stream::ReadWrite;

/// Session state after all phases complete
pub struct Session {
    pub encryption_context: EncryptionContext,
}

impl Session {
    /// Create a new session from session keys
    pub fn new(session_keys: SessionKeys) -> Result<Self> {
        let encryption_context = EncryptionContext::new(&session_keys)
            .context("Failed to create encryption context")?;
        
        Ok(Self {
            encryption_context,
        })
    }

    /// Send encrypted data
    pub fn send_encrypted(&mut self, stream: &mut dyn ReadWrite, data: &[u8]) -> Result<()> {
        println!("[Phase 6] Encrypting and sending data ({} bytes)", data.len());
        
        let encrypted = self.encryption_context.encrypt(data)
            .context("Failed to encrypt data")?;
        
        // Send encrypted data as packet
        let mut packet_data = Vec::new();
        packet_data.extend_from_slice(&(encrypted.len() as u32).to_be_bytes());
        packet_data.extend_from_slice(&encrypted);
        
        let packet = Packet::new(packet_data);
        packet.write(stream)
            .context("Failed to send encrypted packet")?;
        
        Ok(())
    }

    /// Receive and decrypt data
    pub fn receive_encrypted(&mut self, stream: &mut dyn ReadWrite) -> Result<Vec<u8>> {
        let encrypted_packet = Packet::read(stream)
            .context("Failed to receive encrypted packet")?;
        
        if encrypted_packet.payload.len() < 4 {
            anyhow::bail!("Invalid encrypted packet");
        }
        
        let encrypted_len = u32::from_be_bytes([
            encrypted_packet.payload[0],
            encrypted_packet.payload[1],
            encrypted_packet.payload[2],
            encrypted_packet.payload[3],
        ]) as usize;
        
        if encrypted_packet.payload.len() < 4 + encrypted_len {
            anyhow::bail!("Invalid encrypted data length");
        }
        
        let encrypted_data = &encrypted_packet.payload[4..4 + encrypted_len];
        
        println!("[Phase 6] Receiving and decrypting data ({} bytes)", encrypted_data.len());
        
        let decrypted = self.encryption_context.decrypt(encrypted_data)
            .context("Failed to decrypt data")?;
        
        Ok(decrypted)
    }
}

/// Negotiate algorithms (simplified - just agree on AES-256-GCM)
pub fn negotiate_algorithms() -> Result<()> {
    println!("[Phase 6] Negotiating algorithms...");
    println!("[Phase 6] Encryption: AES-256-GCM");
    println!("[Phase 6] MAC: Integrated in GCM");
    println!("[Phase 6] Algorithm negotiation complete");
    Ok(())
}


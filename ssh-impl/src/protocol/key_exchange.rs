use anyhow::{Result, Context};
use crate::crypto::dh::{EphemeralKeyPair, derive_session_keys, SessionKeys};
use crate::utils::packet::Packet;
use crate::utils::stream::ReadWrite;

/// Perform key exchange (server side)
pub fn server_key_exchange(
    stream: &mut dyn ReadWrite,
) -> Result<SessionKeys> {
    println!("[Phase 3] Starting key exchange (server side)...");
    
    // Generate server ephemeral key pair
    let server_keypair = EphemeralKeyPair::generate()
        .context("Failed to generate server key pair")?;
    println!("[Phase 3] Generated server ephemeral key pair");
    
    // Send server public key to client
    let mut server_pubkey_packet = Vec::new();
    server_pubkey_packet.extend_from_slice(&(server_keypair.public_key.len() as u32).to_be_bytes());
    server_pubkey_packet.extend_from_slice(&server_keypair.public_key);
    
    let packet = Packet::new(server_pubkey_packet);
    packet.write(stream)
        .context("Failed to send server public key")?;
    println!("[Phase 3] Sent server public key ({} bytes)", server_keypair.public_key.len());
    
    // Receive client public key
    let client_pubkey_packet = Packet::read(stream)
        .context("Failed to receive client public key")?;
    
    if client_pubkey_packet.payload.len() < 4 {
        anyhow::bail!("Invalid client public key packet");
    }
    
    let key_len = u32::from_be_bytes([
        client_pubkey_packet.payload[0],
        client_pubkey_packet.payload[1],
        client_pubkey_packet.payload[2],
        client_pubkey_packet.payload[3],
    ]) as usize;
    
    if client_pubkey_packet.payload.len() < 4 + key_len {
        anyhow::bail!("Invalid client public key length");
    }
    
    let client_public_key = &client_pubkey_packet.payload[4..4 + key_len];
    println!("[Phase 3] Received client public key ({} bytes)", client_public_key.len());
    
    // Compute shared secret
    let shared_secret = EphemeralKeyPair::compute_shared_secret(
        server_keypair.private_key,
        client_public_key,
    ).context("Failed to compute shared secret")?;
    println!("[Phase 3] Computed shared secret ({} bytes)", shared_secret.len());
    
    // Derive session keys
    let session_keys = derive_session_keys(&shared_secret)
        .context("Failed to derive session keys")?;
    println!("[Phase 3] Derived session keys (encryption, MAC, IV)");
    
    Ok(session_keys)
}

/// Perform key exchange (client side)
pub fn client_key_exchange(
    stream: &mut dyn ReadWrite,
) -> Result<SessionKeys> {
    println!("[Phase 3] Starting key exchange (client side)...");
    
    // Receive server public key
    let server_pubkey_packet = Packet::read(stream)
        .context("Failed to receive server public key")?;
    
    if server_pubkey_packet.payload.len() < 4 {
        anyhow::bail!("Invalid server public key packet");
    }
    
    let key_len = u32::from_be_bytes([
        server_pubkey_packet.payload[0],
        server_pubkey_packet.payload[1],
        server_pubkey_packet.payload[2],
        server_pubkey_packet.payload[3],
    ]) as usize;
    
    if server_pubkey_packet.payload.len() < 4 + key_len {
        anyhow::bail!("Invalid server public key length");
    }
    
    let server_public_key = &server_pubkey_packet.payload[4..4 + key_len];
    println!("[Phase 3] Received server public key ({} bytes)", server_public_key.len());
    
    // Generate client ephemeral key pair
    let client_keypair = EphemeralKeyPair::generate()
        .context("Failed to generate client key pair")?;
    println!("[Phase 3] Generated client ephemeral key pair");
    
    // Send client public key to server
    let mut client_pubkey_packet = Vec::new();
    client_pubkey_packet.extend_from_slice(&(client_keypair.public_key.len() as u32).to_be_bytes());
    client_pubkey_packet.extend_from_slice(&client_keypair.public_key);
    
    let packet = Packet::new(client_pubkey_packet);
    packet.write(stream)
        .context("Failed to send client public key")?;
    println!("[Phase 3] Sent client public key ({} bytes)", client_keypair.public_key.len());
    
    // Compute shared secret
    let shared_secret = EphemeralKeyPair::compute_shared_secret(
        client_keypair.private_key,
        server_public_key,
    ).context("Failed to compute shared secret")?;
    println!("[Phase 3] Computed shared secret ({} bytes)", shared_secret.len());
    
    // Derive session keys
    let session_keys = derive_session_keys(&shared_secret)
        .context("Failed to derive session keys")?;
    println!("[Phase 3] Derived session keys (encryption, MAC, IV)");
    
    Ok(session_keys)
}


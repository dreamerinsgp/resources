use std::path::PathBuf;
use anyhow::{Result, Context};
use crate::crypto::keys::{HostKeyPair, get_ssh_edu_dir};
use crate::utils::packet::Packet;
use crate::utils::stream::ReadWrite;

/// Send host key to client (server side)
pub fn send_host_key(stream: &mut dyn ReadWrite, host_key: &HostKeyPair) -> Result<()> {
    println!("[Phase 4] Sending host key to client...");
    
    let public_key_bytes = host_key.public_key_bytes();
    let mut host_key_packet = Vec::new();
    host_key_packet.extend_from_slice(&(public_key_bytes.len() as u32).to_be_bytes());
    host_key_packet.extend_from_slice(&public_key_bytes);
    
    let packet = Packet::new(host_key_packet);
    packet.write(stream)
        .context("Failed to send host key")?;
    
    println!("[Phase 4] Sent host key ({} bytes)", public_key_bytes.len());
    Ok(())
}

/// Receive and verify host key (client side)
pub fn receive_and_verify_host_key(
    stream: &mut dyn ReadWrite,
    hostname: &str,
) -> Result<Vec<u8>> {
    println!("[Phase 4] Receiving host key from server...");
    
    let host_key_packet = Packet::read(stream)
        .context("Failed to receive host key")?;
    
    if host_key_packet.payload.len() < 4 {
        anyhow::bail!("Invalid host key packet");
    }
    
    let key_len = u32::from_be_bytes([
        host_key_packet.payload[0],
        host_key_packet.payload[1],
        host_key_packet.payload[2],
        host_key_packet.payload[3],
    ]) as usize;
    
    if host_key_packet.payload.len() < 4 + key_len {
        anyhow::bail!("Invalid host key length");
    }
    
    let host_key_bytes = host_key_packet.payload[4..4 + key_len].to_vec();
    println!("[Phase 4] Received host key ({} bytes)", host_key_bytes.len());
    
    // Check known_hosts file
    let ssh_edu_dir = get_ssh_edu_dir()?;
    let known_hosts_path = ssh_edu_dir.join("known_hosts");
    
    if known_hosts_path.exists() {
        // Verify against known hosts
        let known_hosts = std::fs::read_to_string(&known_hosts_path)
            .context("Failed to read known_hosts")?;
        
        let expected_key = hex::encode(&host_key_bytes);
        let host_entry = format!("{} {}", hostname, expected_key);
        
        if known_hosts.lines().any(|line| line == host_entry) {
            println!("[Phase 4] Host key verified against known_hosts");
        } else {
            println!("[Phase 4] WARNING: Host key not found in known_hosts!");
            println!("[Phase 4] This might be a man-in-the-middle attack!");
            println!("[Phase 4] Adding to known_hosts (first connection)");
            
            // Add to known_hosts
            let mut known_hosts_content = known_hosts;
            if !known_hosts_content.ends_with('\n') {
                known_hosts_content.push('\n');
            }
            known_hosts_content.push_str(&host_entry);
            known_hosts_content.push('\n');
            
            std::fs::write(&known_hosts_path, known_hosts_content)
                .context("Failed to write known_hosts")?;
        }
    } else {
        // First connection - create known_hosts
        println!("[Phase 4] First connection to this host");
        println!("[Phase 4] Storing host key in known_hosts");
        
        std::fs::create_dir_all(&ssh_edu_dir)
            .context("Failed to create .ssh_edu directory")?;
        
        let host_entry = format!("{} {}\n", hostname, hex::encode(&host_key_bytes));
        std::fs::write(&known_hosts_path, host_entry)
            .context("Failed to write known_hosts")?;
    }
    
    // Send acknowledgment
    let ack_packet = Packet::new(b"OK".to_vec());
    ack_packet.write(stream)
        .context("Failed to send host key acknowledgment")?;
    
    println!("[Phase 4] Server authentication complete");
    Ok(host_key_bytes)
}

/// Receive host key acknowledgment (server side)
pub fn receive_host_key_ack(stream: &mut dyn ReadWrite) -> Result<()> {
    let ack_packet = Packet::read(stream)
        .context("Failed to receive host key acknowledgment")?;
    
    if ack_packet.payload != b"OK" {
        anyhow::bail!("Host key not accepted by client");
    }
    
    println!("[Phase 4] Client accepted host key");
    Ok(())
}


use std::path::PathBuf;
use anyhow::{Result, Context};
use crate::crypto::keys::{UserKeyPair, verify_signature, get_ssh_edu_dir};
use crate::utils::packet::Packet;
use crate::utils::stream::ReadWrite;

/// Authentication methods
#[derive(Debug, Clone)]
pub enum AuthMethod {
    Password(String),
    PublicKey(Vec<u8>),
}

/// Send authentication request (client side)
pub fn send_auth_request(
    stream: &mut dyn ReadWrite,
    username: &str,
    method: &AuthMethod,
) -> Result<()> {
    println!("[Phase 5] Sending authentication request...");
    
    let mut auth_packet = Vec::new();
    
    // Add username
    auth_packet.extend_from_slice(&(username.len() as u32).to_be_bytes());
    auth_packet.extend_from_slice(username.as_bytes());
    
    match method {
        AuthMethod::Password(password) => {
            auth_packet.push(0); // Method: password
            auth_packet.extend_from_slice(&(password.len() as u32).to_be_bytes());
            auth_packet.extend_from_slice(password.as_bytes());
            println!("[Phase 5] Using password authentication");
        }
        AuthMethod::PublicKey(public_key) => {
            auth_packet.push(1); // Method: public key
            auth_packet.extend_from_slice(&(public_key.len() as u32).to_be_bytes());
            auth_packet.extend_from_slice(public_key);
            println!("[Phase 5] Using public key authentication");
        }
    }
    
    let packet = Packet::new(auth_packet);
    packet.write(stream)
        .context("Failed to send authentication request")?;
    
    // Receive authentication response
    let response_packet = Packet::read(stream)
        .context("Failed to receive authentication response")?;
    
    if response_packet.payload == b"SUCCESS" {
        println!("[Phase 5] Authentication successful!");
        Ok(())
    } else if response_packet.payload == b"FAILURE" {
        anyhow::bail!("Authentication failed");
    } else {
        anyhow::bail!("Invalid authentication response");
    }
}

/// Handle authentication request (server side)
pub fn handle_auth_request(
    stream: &mut dyn ReadWrite,
) -> Result<String> {
    println!("[Phase 5] Receiving authentication request...");
    
    let auth_packet = Packet::read(stream)
        .context("Failed to receive authentication request")?;
    
    if auth_packet.payload.len() < 4 {
        anyhow::bail!("Invalid authentication packet");
    }
    
    // Parse username
    let username_len = u32::from_be_bytes([
        auth_packet.payload[0],
        auth_packet.payload[1],
        auth_packet.payload[2],
        auth_packet.payload[3],
    ]) as usize;
    
    if auth_packet.payload.len() < 4 + username_len {
        anyhow::bail!("Invalid username length");
    }
    
    let username = String::from_utf8(
        auth_packet.payload[4..4 + username_len].to_vec()
    ).context("Invalid username encoding")?;
    
    println!("[Phase 5] Authenticating user: {}", username);
    
    // Parse authentication method
    let method_offset = 4 + username_len;
    if auth_packet.payload.len() <= method_offset {
        anyhow::bail!("Missing authentication method");
    }
    
    let method = auth_packet.payload[method_offset];
    let mut authenticated = false;
    
    match method {
        0 => {
            // Password authentication
            if auth_packet.payload.len() < method_offset + 1 + 4 {
                anyhow::bail!("Invalid password packet");
            }
            
            let password_len = u32::from_be_bytes([
                auth_packet.payload[method_offset + 1],
                auth_packet.payload[method_offset + 2],
                auth_packet.payload[method_offset + 3],
                auth_packet.payload[method_offset + 4],
            ]) as usize;
            
            if auth_packet.payload.len() < method_offset + 5 + password_len {
                anyhow::bail!("Invalid password length");
            }
            
            let password = String::from_utf8(
                auth_packet.payload[method_offset + 5..method_offset + 5 + password_len].to_vec()
            ).context("Invalid password encoding")?;
            
            authenticated = authenticate_password(&username, &password)?;
        }
        1 => {
            // Public key authentication
            if auth_packet.payload.len() < method_offset + 1 + 4 {
                anyhow::bail!("Invalid public key packet");
            }
            
            let key_len = u32::from_be_bytes([
                auth_packet.payload[method_offset + 1],
                auth_packet.payload[method_offset + 2],
                auth_packet.payload[method_offset + 3],
                auth_packet.payload[method_offset + 4],
            ]) as usize;
            
            if auth_packet.payload.len() < method_offset + 5 + key_len {
                anyhow::bail!("Invalid public key length");
            }
            
            let public_key = auth_packet.payload[method_offset + 5..method_offset + 5 + key_len].to_vec();
            
            authenticated = authenticate_public_key(&username, &public_key)?;
        }
        _ => {
            anyhow::bail!("Unknown authentication method: {}", method);
        }
    }
    
    // Send authentication response
    let response = if authenticated {
        println!("[Phase 5] Authentication successful!");
        b"SUCCESS".to_vec()
    } else {
        println!("[Phase 5] Authentication failed!");
        b"FAILURE".to_vec()
    };
    
    let response_packet = Packet::new(response);
    response_packet.write(stream)
        .context("Failed to send authentication response")?;
    
    if authenticated {
        Ok(username)
    } else {
        anyhow::bail!("Authentication failed")
    }
}

/// Authenticate user with password (simplified - in production, use proper password hashing)
fn authenticate_password(username: &str, password: &str) -> Result<bool> {
    // Simplified: check against a simple user database
    // In production, use proper password hashing (bcrypt, argon2, etc.)
    let ssh_edu_dir = get_ssh_edu_dir()?;
    let users_file = ssh_edu_dir.join("users.json");
    
    if !users_file.exists() {
        // Create default user for testing
        let default_users = serde_json::json!({
            "testuser": "testpass"
        });
        std::fs::create_dir_all(&ssh_edu_dir)?;
        std::fs::write(&users_file, serde_json::to_string_pretty(&default_users)?)?;
    }
    
    let users_data: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&users_file)?
    )?;
    
    if let Some(stored_password) = users_data.get(username).and_then(|v| v.as_str()) {
        Ok(stored_password == password)
    } else {
        Ok(false)
    }
}

/// Authenticate user with public key
fn authenticate_public_key(username: &str, public_key: &[u8]) -> Result<bool> {
    let ssh_edu_dir = get_ssh_edu_dir()?;
    let authorized_keys_file = ssh_edu_dir.join(format!("authorized_keys_{}", username));
    
    if !authorized_keys_file.exists() {
        return Ok(false);
    }
    
    let authorized_keys_content = std::fs::read_to_string(&authorized_keys_file)?;
    let public_key_hex = hex::encode(public_key);
    
    // Check if public key is in authorized_keys
    Ok(authorized_keys_content.lines().any(|line| {
        line.trim() == public_key_hex
    }))
}


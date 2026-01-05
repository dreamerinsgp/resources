use std::io::{Read, Write};
use anyhow::{Result, Context};
use crate::utils::io;

/// SSH protocol version string
const SERVER_VERSION: &str = "SSH-2.0-EduSSH-1.0";
const CLIENT_VERSION: &str = "SSH-2.0-EduSSH-Client-1.0";

/// Send protocol version string
pub fn send_version_string(stream: &mut dyn Write, is_server: bool) -> Result<()> {
    let version = if is_server {
        SERVER_VERSION
    } else {
        CLIENT_VERSION
    };
    
    let version_line = format!("{}\r\n", version);
    io::write_all(stream, version_line.as_bytes())
        .context("Failed to send version string")?;
    
    println!("[Phase 2] Sent version: {}", version);
    Ok(())
}

/// Receive and parse protocol version string
pub fn receive_version_string(stream: &mut dyn Read) -> Result<String> {
    let version_line = io::read_line_crlf(stream)
        .context("Failed to receive version string")?;
    
    println!("[Phase 2] Received version: {}", version_line);
    Ok(version_line)
}

/// Parse protocol version from version string
pub fn parse_version(version_string: &str) -> Result<u8> {
    // Extract version number from "SSH-2.0-..." format
    if let Some(version_part) = version_string.strip_prefix("SSH-") {
        if let Some(major_version) = version_part.split('-').next() {
            if let Some(major) = major_version.split('.').next() {
                return major.parse::<u8>()
                    .context("Invalid version number");
            }
        }
    }
    
    anyhow::bail!("Invalid version string format: {}", version_string)
}

/// Negotiate protocol version between client and server
pub fn negotiate_version(client_version: &str, server_version: &str) -> Result<()> {
    let client_major = parse_version(client_version)
        .context("Failed to parse client version")?;
    let server_major = parse_version(server_version)
        .context("Failed to parse server version")?;
    
    if client_major != server_major {
        anyhow::bail!(
            "Version mismatch: client={}, server={}",
            client_major,
            server_major
        );
    }
    
    if client_major != 2 {
        anyhow::bail!("Only SSH-2.0 is supported");
    }
    
    println!("[Phase 2] Version negotiation successful: SSH-2.0");
    Ok(())
}


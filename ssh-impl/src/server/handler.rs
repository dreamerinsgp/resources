use std::net::TcpStream;
use anyhow::{Result, Context};
use crate::protocol::version::{send_version_string, receive_version_string, negotiate_version};
use crate::protocol::key_exchange::server_key_exchange;
use crate::protocol::server_auth::{send_host_key, receive_host_key_ack};
use crate::protocol::client_auth::handle_auth_request;
use crate::protocol::session::{Session, negotiate_algorithms};
use crate::crypto::keys::{HostKeyPair, get_ssh_edu_dir};
use crate::utils::stream::ReadWrite;

/// Handle a single SSH connection
pub fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut stream_ref: Box<dyn ReadWrite> = Box::new(stream);
    
    println!("\n[Phase 1] TCP connection established");
    
    // Phase 2: Protocol Version Exchange
    println!("\n=== Phase 2: Protocol Version Exchange ===");
    send_version_string(&mut *stream_ref, true)?;
    let client_version = receive_version_string(&mut *stream_ref)?;
    let server_version = "SSH-2.0-EduSSH-1.0";
    negotiate_version(&client_version, server_version)?;
    
    // Phase 3: Key Exchange
    println!("\n=== Phase 3: Key Exchange ===");
    let session_keys = server_key_exchange(&mut *stream_ref)?;
    
    // Phase 4: Server Authentication
    println!("\n=== Phase 4: Server Authentication ===");
    let ssh_edu_dir = get_ssh_edu_dir()?;
    let host_key_path = ssh_edu_dir.join("host_key");
    let host_key = HostKeyPair::load_or_generate(&host_key_path)?;
    send_host_key(&mut *stream_ref, &host_key)?;
    receive_host_key_ack(&mut *stream_ref)?;
    
    // Phase 5: Client Authentication
    println!("\n=== Phase 5: Client Authentication ===");
    let username = handle_auth_request(&mut *stream_ref)?;
    
    // Phase 6: Session Establishment
    println!("\n=== Phase 6: Session Establishment ===");
    negotiate_algorithms()?;
    let mut session = Session::new(session_keys)?;
    
    println!("\n=== SSH Session Established ===");
    println!("Authenticated user: {}", username);
    println!("Secure channel ready for data transmission");
    
    // Simple echo server for demonstration
    println!("\nEcho server mode - type messages to echo back");
    loop {
        match session.receive_encrypted(&mut *stream_ref) {
            Ok(data) => {
                let message = String::from_utf8_lossy(&data);
                println!("Received: {}", message);
                
                if message.trim() == "exit" {
                    println!("Client requested disconnect");
                    break;
                }
                
                let response = format!("Echo: {}", message);
                session.send_encrypted(&mut *stream_ref, response.as_bytes())?;
            }
            Err(e) => {
                println!("Connection closed or error: {}", e);
                break;
            }
        }
    }
    
    println!("Connection closed");
    Ok(())
}


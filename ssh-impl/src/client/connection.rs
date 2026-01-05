use std::net::TcpStream;
use std::io::{self, Write};
use anyhow::{Result, Context};
use crate::protocol::version::{send_version_string, receive_version_string, negotiate_version};
use crate::protocol::key_exchange::client_key_exchange;
use crate::protocol::server_auth::receive_and_verify_host_key;
use crate::protocol::client_auth::{send_auth_request, AuthMethod};
use crate::protocol::session::{Session, negotiate_algorithms};
use crate::utils::stream::ReadWrite;

/// Connect to SSH server
pub fn connect(host: &str, port: u16, username: &str) -> Result<()> {
    let address = format!("{}:{}", host, port);
    println!("Connecting to {}...", address);
    
    let stream = TcpStream::connect(&address)
        .context(format!("Failed to connect to {}", address))?;
    
    let mut stream_ref: Box<dyn ReadWrite> = Box::new(stream);
    
    println!("\n[Phase 1] TCP connection established");
    
    // Phase 2: Protocol Version Exchange
    println!("\n=== Phase 2: Protocol Version Exchange ===");
    let server_version = receive_version_string(&mut *stream_ref)?;
    send_version_string(&mut *stream_ref, false)?;
    let client_version = "SSH-2.0-EduSSH-Client-1.0";
    negotiate_version(client_version, &server_version)?;
    
    // Phase 3: Key Exchange
    println!("\n=== Phase 3: Key Exchange ===");
    let session_keys = client_key_exchange(&mut *stream_ref)?;
    
    // Phase 4: Server Authentication
    println!("\n=== Phase 4: Server Authentication ===");
    receive_and_verify_host_key(&mut *stream_ref, host)?;
    
    // Phase 5: Client Authentication
    println!("\n=== Phase 5: Client Authentication ===");
    // Try password authentication first (simplified)
    println!("Attempting password authentication...");
    print!("Password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();
    
    send_auth_request(&mut *stream_ref, username, &AuthMethod::Password(password))?;
    
    // Phase 6: Session Establishment
    println!("\n=== Phase 6: Session Establishment ===");
    negotiate_algorithms()?;
    let mut session = Session::new(session_keys)?;
    
    println!("\n=== SSH Session Established ===");
    println!("Connected to {} as {}", host, username);
    println!("Secure channel ready for data transmission");
    
    // Simple interactive client
    println!("\nInteractive mode - type messages (type 'exit' to quit)");
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "exit" {
            break;
        }
        
        session.send_encrypted(&mut *stream_ref, input.as_bytes())?;
        
        match session.receive_encrypted(&mut *stream_ref) {
            Ok(response) => {
                let message = String::from_utf8_lossy(&response);
                println!("{}", message);
            }
            Err(e) => {
                println!("Error receiving response: {}", e);
                break;
            }
        }
    }
    
    println!("Disconnected");
    Ok(())
}


use std::net::{TcpListener, TcpStream};
use std::io;
use anyhow::{Result, Context};
use crate::server::handler::handle_connection;

/// Run SSH server on specified port
pub fn run(port: u16) -> Result<()> {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)
        .context(format!("Failed to bind to {}", address))?;
    
    println!("SSH Server listening on {}", address);
    println!("Ready to accept connections...");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr()
                    .map(|addr| addr.to_string())
                    .unwrap_or_else(|_| "unknown".to_string());
                println!("\n=== New connection from {} ===", peer_addr);
                
                // Handle each connection (in production, spawn a thread)
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Connection error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    
    Ok(())
}


use ssh_impl::client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <host> <port> <username>", args[0]);
        std::process::exit(1);
    }
    
    let host = &args[1];
    let port: u16 = args[2].parse().unwrap_or(2222);
    let username = &args[3];
    
    if let Err(e) = client::connect(host, port, username) {
        eprintln!("Client error: {}", e);
        std::process::exit(1);
    }
}


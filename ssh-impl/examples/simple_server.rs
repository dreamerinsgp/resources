use ssh_impl::server;

fn main() {
    let port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2222);
    
    if let Err(e) = server::run(port) {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}


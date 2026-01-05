# SSH Server and Client Implementation Roadmap

## Overview

Build a simplified educational SSH implementation in Rust that demonstrates all 6 phases of SSH connection establishment. The implementation will use existing crypto libraries (`ring`, `ed25519-dalek`, `rsa`) and focus on clarity and educational value while maintaining security.

## Project Structure

```javascript
ssh-impl/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                 # Entry point with CLI for client/server
│   ├── lib.rs                  # Library exports
│   ├── protocol/
│   │   ├── mod.rs              # Protocol module
│   │   ├── version.rs          # Phase 2: Protocol version exchange
│   │   ├── key_exchange.rs     # Phase 3: Diffie-Hellman key exchange
│   │   ├── server_auth.rs      # Phase 4: Server authentication
│   │   ├── client_auth.rs      # Phase 5: Client authentication
│   │   └── session.rs          # Phase 6: Session establishment
│   ├── server/
│   │   ├── mod.rs              # Server module
│   │   ├── tcp.rs              # Phase 1: TCP server setup
│   │   └── handler.rs          # Connection handler
│   ├── client/
│   │   ├── mod.rs              # Client module
│   │   └── connection.rs       # Client connection logic
│   ├── crypto/
│   │   ├── mod.rs              # Crypto utilities
│   │   ├── dh.rs               # Diffie-Hellman implementation
│   │   ├── keys.rs             # Key generation and management
│   │   └── encryption.rs       # Encryption/decryption utilities
│   └── utils/
│       ├── mod.rs
│       ├── packet.rs           # SSH packet encoding/decoding
│       └── io.rs               # Network I/O helpers
└── examples/
    ├── simple_server.rs        # Example server usage
    └── simple_client.rs        # Example client usage
```



## Implementation Phases

### Phase 1: TCP Connection (`server/tcp.rs`, `client/connection.rs`)

**Server:**

- Create TCP listener on configurable port (default 2222)
- Accept incoming connections

- Spawn handler for each connection

**Client:**

- Connect to server via TCP

- Establish socket connection

**Dependencies:** `std::net::TcpListener`, `std::net::TcpStream`

### Phase 2: Protocol Version Exchange (`protocol/version.rs`)

**Implementation:**

- Server sends version string: `SSH-2.0-EduSSH-1.0\r\n`
- Client sends version string: `SSH-2.0-EduSSH-Client-1.0\r\n`
- Parse and validate version strings

- Verify compatibility (both must be SSH-2.0)

**Key Functions:**

- `send_version_string(stream)` - Send version string

- `receive_version_string(stream)` - Receive and parse version string

- `negotiate_version(client_ver, server_ver)` - Verify compatibility

### Phase 3: Key Exchange (`protocol/key_exchange.rs`, `crypto/dh.rs`)

**Implementation:**

- Use `ring::agreement` for Diffie-Hellman key exchange

- Generate ephemeral key pairs (X25519)
- Exchange public keys
- Compute shared secret

- Derive session keys using HKDF

**Key Functions:**

- `generate_ephemeral_keypair()` - Generate temporary key pair

- `compute_shared_secret(private_key, peer_public_key)` - Compute shared secret

- `derive_session_keys(shared_secret)` - Derive encryption/MAC keys using HKDF

**Dependencies:** `ring` crate for X25519 and HKDF

### Phase 4: Server Authentication (`protocol/server_auth.rs`)

**Implementation:**

- Server generates host key pair (Ed25519) on first run
- Store host key in `~/.ssh_edu/host_key`

- Server sends host public key to client

- Client stores host key in `~/.ssh_edu/known_hosts` (first connection)

- Client verifies host key matches stored key (subsequent connections)

**Key Functions:**

- `generate_host_key()` - Generate server host key

- `load_or_generate_host_key()` - Load existing or generate new

- `send_host_key(stream, host_key)` - Send host public key

- `verify_host_key(host_key, known_hosts)` - Verify host key

**Dependencies:** `ed25519-dalek` for Ed25519 keys

### Phase 5: Client Authentication (`protocol/client_auth.rs`)

**Implementation:**

- Support password authentication (simplified)

- Support public key authentication (Ed25519)

- Server stores authorized keys in `~/.ssh_edu/authorized_keys`
- Client sends authentication request

- Server verifies and responds

**Key Functions:**

- `authenticate_password(username, password)` - Password auth

- `authenticate_public_key(username, public_key)` - Public key auth

- `send_auth_request(stream, method, credentials)` - Send auth request
- `handle_auth_request(stream)` - Server handles auth

**Dependencies:** `ed25519-dalek` for signature verification

### Phase 6: Session Establishment (`protocol/session.rs`, `crypto/encryption.rs`)

**Implementation:**

- Negotiate encryption algorithm (AES-256-GCM)
- Negotiate MAC algorithm (integrated in GCM)

- Set up encryption/decryption contexts
- Implement packet encryption/decryption

- Enable secure data transmission

**Key Functions:**

- `negotiate_algorithms()` - Agree on crypto algorithms

- `setup_encryption(session_keys)` - Initialize encryption contexts
- `encrypt_packet(data)` - Encrypt outgoing packet

- `decrypt_packet(encrypted_data)` - Decrypt incoming packet

**Dependencies:** `ring` for AES-GCM encryption

## Dependencies (`Cargo.toml`)

```toml
[dependencies]
ring = "0.17"              # Crypto primitives (DH, HKDF, AES-GCM)
ed25519-dalek = "2.1"     # Ed25519 signatures
rsa = "0.9"               # RSA (optional, for RSA keys)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"        # Key storage format
clap = { version = "4.0", features = ["derive"] }  # CLI
anyhow = "1.0"            # Error handling
tokio = { version = "1.0", features = ["full"] }   # Async runtime
```



## Implementation Steps

### Step 1: Project Setup

- Initialize Rust project with `cargo new ssh-impl`

- Add dependencies to `Cargo.toml`
- Create module structure
- Set up basic CLI with `clap` (server/client modes)

### Step 2: Phase 1 - TCP Connection

- Implement TCP server listener
- Implement TCP client connection

- Add basic error handling
- Test TCP connection establishment

### Step 3: Phase 2 - Protocol Version Exchange

- Implement version string format

- Add send/receive version functions

- Implement version negotiation logic
- Add unit tests for version parsing

### Step 4: Phase 3 - Key Exchange

- Implement X25519 key pair generation
- Implement shared secret computation

- Implement HKDF for key derivation
- Add logging for educational visibility

- Test key exchange between client/server

### Step 5: Phase 4 - Server Authentication

- Implement host key generation/storage
- Implement host key exchange

- Implement known_hosts management
- Add host key verification
- Test server authentication

### Step 6: Phase 5 - Client Authentication

- Implement password authentication
- Implement public key authentication

- Add authorized_keys management
- Implement authentication flow

- Test both auth methods

### Step 7: Phase 6 - Session Establishment

- Implement algorithm negotiation

- Implement AES-GCM encryption setup
- Implement packet encryption/decryption

- Add secure channel for data transmission
- Test encrypted communication

### Step 8: Integration and Testing

- Integrate all phases into complete flow

- Add comprehensive logging for educational purposes
- Create example server/client programs

- Add error handling throughout

- Write integration tests

### Step 9: Documentation

- Add inline documentation for each phase
- Create README with usage examples
- Document packet formats
- Add comments explaining cryptographic operations

## Key Design Decisions

1. **Simplified but Secure**: Use production-grade crypto libraries but keep protocol implementation simple

2. **Educational Focus**: Extensive logging and comments to explain each step

3. **Modular Design**: Each phase in separate module for clarity

4. **Non-Standard Port**: Use port 2222 by default to avoid conflicts with system SSH

5. **Key Storage**: Store keys in `~/.ssh_edu/` to avoid conflicts with system SSH keys

6. **Synchronous I/O**: Start with `std::net` for simplicity, can upgrade to async later

## Testing Strategy

- Unit tests for each protocol phase
- Integration tests for complete connection flow

- Manual testing with verbose logging

- Test against different scenarios (first connection, reconnection, auth failures)

## Future Enhancements (Out of Scope)

- Full RFC 4253 compliance

- Multiple encryption algorithms
- Compression support

- Port forwarding
- Interactive shell support
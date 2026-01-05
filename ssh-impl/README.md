# Educational SSH Implementation

A simplified educational SSH server and client implementation in Rust that demonstrates all 6 phases of SSH connection establishment.

## Features

- Phase 1: TCP Connection
- Phase 2: Protocol Version Exchange
- Phase 3: Diffie-Hellman Key Exchange (X25519)
- Phase 4: Server Authentication (Ed25519 host keys)
- Phase 5: Client Authentication (Password and Public Key)
- Phase 6: Session Establishment (AES-256-GCM encryption)

## Usage

### Server

```bash
# Run server on default port 2222
cargo run -- server

# Run server on custom port
cargo run -- server --port 2223
```

### Client

```bash
# Connect to server
cargo run -- client --host localhost --port 2222 --user testuser

# Default password is "testpass" (for user "testuser")
```

## First Run Setup

On first run, the server will:
1. Create `~/.ssh_edu/` directory
2. Generate a host key pair
3. Create a default user database (`users.json`) with user "testuser" and password "testpass"

The client will:
1. Create `~/.ssh_edu/` directory
2. Store server's host key in `known_hosts` on first connection

## Project Structure

- `src/protocol/` - SSH protocol implementation (all 6 phases)
- `src/server/` - SSH server implementation
- `src/client/` - SSH client implementation
- `src/crypto/` - Cryptographic utilities
- `src/utils/` - Helper utilities

## Educational Purpose

This implementation is designed for educational purposes to understand the SSH protocol internals. It uses production-grade crypto libraries (`ring`, `ed25519-dalek`) but keeps the protocol implementation simplified for clarity.

## Security Note

This is an educational implementation and should NOT be used in production environments.


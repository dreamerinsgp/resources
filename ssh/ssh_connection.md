Q1: explain the principle of ssh connection.

## SSH Connection Principles

SSH (Secure Shell) is a cryptographic network protocol that enables secure remote access and data communication over an unsecured network. Here's how it works:

### 1. **Basic Architecture**
- **Client**: The machine initiating the connection
- **Server**: The remote machine accepting the connection
- **SSH Daemon (sshd)**: Service running on the server listening for connections (typically on port 22)

### 2. **Connection Establishment Process**

#### Phase 1: TCP Connection
- Client establishes a TCP connection to the server on port 22 (or configured port)
- Three-way handshake completes the TCP connection

#### Phase 2: Protocol Version Exchange
- Client and server exchange SSH protocol version strings
- Both parties agree on the SSH protocol version (e.g., SSH-2.0)

#### Phase 3: Key Exchange (Diffie-Hellman)
- **Purpose**: Establish a shared secret key without transmitting it over the network
- Both parties generate temporary key pairs
- Exchange public keys and compute shared secret using Diffie-Hellman algorithm
- This shared secret is used to derive session keys for encryption

#### Phase 4: Server Authentication
- Server sends its host key (public key) to the client
- Client verifies server identity:
  - First connection: Client stores server's host key (user confirms)
  - Subsequent connections: Client compares received key with stored key
  - If mismatch detected, security warning is shown (possible MITM attack)

#### Phase 5: Client Authentication
SSH supports multiple authentication methods:

**a) Password Authentication**
- Client sends encrypted password
- Server verifies against system credentials

**b) Public Key Authentication (Recommended)**
- Client has private key, server has corresponding public key
- Server sends a challenge encrypted with client's public key
- Client decrypts challenge with private key and sends response
- Server verifies response

**c) Other Methods**
- Certificate-based authentication
- Kerberos authentication
- Host-based authentication

#### Phase 6: Session Establishment
- After successful authentication, SSH session is established
- Client and server negotiate:
  - Encryption algorithms (AES, ChaCha20, etc.)
  - MAC (Message Authentication Code) algorithms
  - Compression algorithms
  - Session keys derived from the shared secret

### 3. **Security Features**

**Encryption**
- All data transmitted is encrypted using symmetric encryption
- Session keys are derived from the initial key exchange
- Multiple algorithms supported (AES-256, ChaCha20-Poly1305, etc.)

**Integrity**
- MAC ensures data hasn't been tampered with during transmission
- Each packet includes a MAC for verification

**Forward Secrecy**
- Each session uses unique keys
- Compromised session keys don't affect past or future sessions

**Authentication**
- Server authentication prevents man-in-the-middle attacks
- Client authentication ensures only authorized users access the server

### 4. **Key Components**

**Host Keys**
- Server's long-term identity keys (RSA, ECDSA, Ed25519)
- Stored in `/etc/ssh/ssh_host_*_key` on server
- Public keys stored in `~/.ssh/known_hosts` on client

**User Keys**
- Client's authentication keys
- Private key: `~/.ssh/id_rsa` (or id_ed25519, id_ecdsa)
- Public key: `~/.ssh/id_rsa.pub` (copied to server's `~/.ssh/authorized_keys`)

### 5. **Connection Flow Summary**

```
1. Client → Server: TCP connection (port 22)
2. Client ↔ Server: Protocol version negotiation
3. Client ↔ Server: Key exchange (Diffie-Hellman)
4. Server → Client: Host key for server authentication
5. Client → Server: Authentication (password/key/certificate)
6. Client ↔ Server: Encrypted session established
7. Client ↔ Server: Secure data transmission
```

### 6. **Common SSH Commands**

```bash
# Basic connection
ssh user@hostname

# With specific port
ssh -p 2222 user@hostname

# Using specific key
ssh -i ~/.ssh/my_key user@hostname

# Generate key pair
ssh-keygen -t ed25519 -C "your_email@example.com"

# Copy public key to server
ssh-copy-id user@hostname
```

### 7. **Security Best Practices**

- Use public key authentication instead of passwords
- Disable root login via SSH
- Use strong key algorithms (Ed25519 preferred over RSA)
- Implement fail2ban to prevent brute force attacks
- Use SSH keys with passphrases
- Regularly rotate keys
- Keep SSH software updated
- Use non-standard ports (optional, security through obscurity) 



![ssh_connection](ssh_connection.png)
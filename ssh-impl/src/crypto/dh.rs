use ring::agreement;
use ring::rand::{SystemRandom, SecureRandom};
use anyhow::{Result, Context};

/// Ephemeral key pair for Diffie-Hellman key exchange
pub struct EphemeralKeyPair {
    pub private_key: agreement::EphemeralPrivateKey,
    pub public_key: Vec<u8>,
}

impl EphemeralKeyPair {
    /// Generate a new ephemeral key pair using X25519
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let private_key = agreement::EphemeralPrivateKey::generate(
            &agreement::X25519,
            &rng,
        ).map_err(|_| anyhow::anyhow!("Failed to generate ephemeral private key"))?;

        let public_key = private_key.compute_public_key()
            .map_err(|_| anyhow::anyhow!("Failed to compute public key"))?;

        Ok(Self {
            private_key,
            public_key: public_key.as_ref().to_vec(),
        })
    }

    /// Compute shared secret from our private key and peer's public key
    pub fn compute_shared_secret(
        private_key: agreement::EphemeralPrivateKey,
        peer_public_key: &[u8],
    ) -> Result<Vec<u8>> {
        let peer_public_key = agreement::UnparsedPublicKey::new(
            &agreement::X25519,
            peer_public_key,
        );

        let mut shared_secret = vec![0u8; 32];
        agreement::agree_ephemeral(
            private_key,
            &peer_public_key,
            |key_material| {
                shared_secret.copy_from_slice(key_material);
            },
        ).map_err(|_| anyhow::anyhow!("Failed to compute shared secret"))?;

        Ok(shared_secret)
    }
}

/// Derive session keys from shared secret using HKDF
pub fn derive_session_keys(shared_secret: &[u8]) -> Result<SessionKeys> {
    // Simplified key derivation using SHA256 directly
    // In production, use proper HKDF, but for educational purposes this works
    use ring::digest;
    
    // Derive encryption key: SHA256(shared_secret || "encryption")
    let mut enc_input = Vec::new();
    enc_input.extend_from_slice(shared_secret);
    enc_input.extend_from_slice(b"encryption");
    let encryption_key_bytes = digest::digest(&digest::SHA256, &enc_input);
    let mut encryption_key = [0u8; 32];
    encryption_key.copy_from_slice(encryption_key_bytes.as_ref());
    
    // Derive MAC key: SHA256(shared_secret || "mac")
    let mut mac_input = Vec::new();
    mac_input.extend_from_slice(shared_secret);
    mac_input.extend_from_slice(b"mac");
    let mac_key_digest = digest::digest(&digest::SHA256, &mac_input);
    let mut mac_key_bytes = [0u8; 32];
    mac_key_bytes.copy_from_slice(mac_key_digest.as_ref());
    
    // Derive IV: First 12 bytes of SHA256(shared_secret || "iv")
    let mut iv_input = Vec::new();
    iv_input.extend_from_slice(shared_secret);
    iv_input.extend_from_slice(b"iv");
    let iv_digest = digest::digest(&digest::SHA256, &iv_input);
    let mut iv_bytes = [0u8; 12];
    iv_bytes.copy_from_slice(&iv_digest.as_ref()[0..12]);

    Ok(SessionKeys {
        encryption_key,
        mac_key: mac_key_bytes,
        iv: iv_bytes,
    })
}

/// Session keys derived from shared secret
pub struct SessionKeys {
    pub encryption_key: [u8; 32],
    pub mac_key: [u8; 32],
    pub iv: [u8; 12],
}


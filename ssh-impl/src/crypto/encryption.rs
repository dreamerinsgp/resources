use ring::aead::{self, BoundKey};
use anyhow::{Result, Context};
use crate::crypto::dh::SessionKeys;

/// Simple nonce sequence using a counter
struct CounterNonceSequence {
    base_iv: [u8; 12],
    counter: u64,
}

impl aead::NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> Result<aead::Nonce, ring::error::Unspecified> {
        let mut nonce_bytes = self.base_iv;
        // XOR counter into last 8 bytes of nonce
        let counter_bytes = self.counter.to_be_bytes();
        for i in 0..8 {
            nonce_bytes[4 + i] ^= counter_bytes[i];
        }
        self.counter += 1;
        Ok(aead::Nonce::assume_unique_for_key(nonce_bytes))
    }
}

/// Encryption context for encrypting/decrypting packets
pub struct EncryptionContext {
    sealing_key: aead::SealingKey<CounterNonceSequence>,
    opening_key: aead::OpeningKey<CounterNonceSequence>,
}

impl EncryptionContext {
    /// Create a new encryption context from session keys
    pub fn new(session_keys: &SessionKeys) -> Result<Self> {
        let unbound_sealing_key = aead::UnboundKey::new(
            &aead::AES_256_GCM,
            &session_keys.encryption_key,
        ).map_err(|_| anyhow::anyhow!("Failed to create sealing key"))?;

        let unbound_opening_key = aead::UnboundKey::new(
            &aead::AES_256_GCM,
            &session_keys.encryption_key,
        ).map_err(|_| anyhow::anyhow!("Failed to create opening key"))?;

        let sealing_nonce_sequence = CounterNonceSequence {
            base_iv: session_keys.iv,
            counter: 0,
        };
        
        let opening_nonce_sequence = CounterNonceSequence {
            base_iv: session_keys.iv,
            counter: 0,
        };

        let sealing_key = aead::SealingKey::new(unbound_sealing_key, sealing_nonce_sequence);
        let opening_key = aead::OpeningKey::new(unbound_opening_key, opening_nonce_sequence);

        Ok(Self {
            sealing_key,
            opening_key,
        })
    }

    /// Encrypt a packet
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut in_out = plaintext.to_vec();
        let tag = self.sealing_key.seal_in_place_separate_tag(
            aead::Aad::empty(),
            &mut in_out,
        ).map_err(|_| anyhow::anyhow!("Failed to encrypt packet"))?;

        // Append authentication tag
        in_out.extend_from_slice(tag.as_ref());
        Ok(in_out)
    }

    /// Decrypt a packet
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 16 {
            anyhow::bail!("Ciphertext too short");
        }

        let mut in_out = ciphertext.to_vec();
        self.opening_key.open_in_place(
            aead::Aad::empty(),
            &mut in_out,
        ).map_err(|_| anyhow::anyhow!("Failed to decrypt packet"))?;

        // Remove authentication tag (last 16 bytes)
        let plaintext_len = in_out.len() - 16;
        Ok(in_out[..plaintext_len].to_vec())
    }
}


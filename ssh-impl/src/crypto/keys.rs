use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand_core::OsRng;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// Host key pair for server authentication
pub struct HostKeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl HostKeyPair {
    /// Generate a new host key pair
    pub fn generate() -> Result<Self> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Load host key from file or generate if it doesn't exist
    pub fn load_or_generate(key_path: &Path) -> Result<Self> {
        if key_path.exists() {
            Self::load(key_path)
        } else {
            let key_pair = Self::generate()?;
            key_pair.save(key_path)?;
            Ok(key_pair)
        }
    }

    /// Load host key from file
    pub fn load(key_path: &Path) -> Result<Self> {
        let data = fs::read(key_path)
            .context("Failed to read host key file")?;
        
        let key_data: KeyData = serde_json::from_slice(&data)
            .context("Failed to parse host key file")?;
        
        if key_data.private_key.len() != 32 {
            anyhow::bail!("Invalid private key length: expected 32 bytes");
        }
        
        let private_key_array: [u8; 32] = key_data.private_key.try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert private key to array"))?;
        let signing_key = SigningKey::from_bytes(&private_key_array);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Save host key to file
    pub fn save(&self, key_path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = key_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create key directory")?;
        }

        let key_data = KeyData {
            private_key: self.signing_key.to_bytes().to_vec(),
            public_key: self.verifying_key.to_bytes().to_vec(),
        };

        let json = serde_json::to_string_pretty(&key_data)
            .context("Failed to serialize key data")?;
        
        fs::write(key_path, json)
            .context("Failed to write host key file")?;
        
        Ok(())
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.verifying_key.to_bytes().to_vec()
    }

    /// Sign data with the host key
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.signing_key.sign(data)
    }
}

/// User key pair for client authentication
pub struct UserKeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl UserKeyPair {
    /// Generate a new user key pair
    pub fn generate() -> Result<Self> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Load user key from file or generate if it doesn't exist
    pub fn load_or_generate(key_path: &Path) -> Result<Self> {
        if key_path.exists() {
            Self::load(key_path)
        } else {
            let key_pair = Self::generate()?;
            key_pair.save(key_path)?;
            Ok(key_pair)
        }
    }

    /// Load user key from file
    pub fn load(key_path: &Path) -> Result<Self> {
        let data = fs::read(key_path)
            .context("Failed to read user key file")?;
        
        let key_data: KeyData = serde_json::from_slice(&data)
            .context("Failed to parse user key file")?;
        
        if key_data.private_key.len() != 32 {
            anyhow::bail!("Invalid private key length: expected 32 bytes");
        }
        
        let private_key_array: [u8; 32] = key_data.private_key.try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert private key to array"))?;
        let signing_key = SigningKey::from_bytes(&private_key_array);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Save user key to file
    pub fn save(&self, key_path: &Path) -> Result<()> {
        if let Some(parent) = key_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create key directory")?;
        }

        let key_data = KeyData {
            private_key: self.signing_key.to_bytes().to_vec(),
            public_key: self.verifying_key.to_bytes().to_vec(),
        };

        let json = serde_json::to_string_pretty(&key_data)
            .context("Failed to serialize key data")?;
        
        fs::write(key_path, json)
            .context("Failed to write user key file")?;
        
        Ok(())
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.verifying_key.to_bytes().to_vec()
    }

    /// Sign data with the user key
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.signing_key.sign(data)
    }
}

/// Verify a signature with a public key
pub fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<()> {
    if public_key.len() != 32 {
        anyhow::bail!("Invalid public key length: expected 32 bytes");
    }
    if signature.len() != 64 {
        anyhow::bail!("Invalid signature length: expected 64 bytes");
    }
    
    let public_key_array: [u8; 32] = public_key.try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert public key to array"))?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)
        .map_err(|e| anyhow::anyhow!("Invalid public key bytes: {}", e))?;
    
    let signature_array: [u8; 64] = signature.try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert signature to array"))?;
    let sig = Signature::from_bytes(&signature_array);
    
    verifying_key.verify(message, &sig)
        .map_err(|e| anyhow::anyhow!("Signature verification failed: {}", e))?;
    
    Ok(())
}

/// Key data structure for serialization
#[derive(serde::Serialize, serde::Deserialize)]
struct KeyData {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

/// Get SSH education directory path
pub fn get_ssh_edu_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".ssh_edu"))
}


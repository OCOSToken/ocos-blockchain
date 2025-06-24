//! # wallet::keystore
//!
//! Provides secure encryption, export, and import of OCOS wallet private keys.
//! Follows a simplified version of Ethereum V3 Keystore (PBKDF2 + AES-256).
//!
//! Use `Keystore::encrypt()` to export and `Keystore::decrypt()` to restore.

use crate::crypto::keypair::{PrivateKey, PublicKey};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM (secure symmetric encryption)
use aes_gcm::aead::{Aead, NewAead};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::{RngCore, rngs::OsRng};
use serde::{Serialize, Deserialize};
use base64::{encode as b64e, decode as b64d};

const PBKDF2_ITERATIONS: u32 = 100_000;
const NONCE_LEN: usize = 12;
const SALT_LEN: usize = 16;

/// Structure for encrypted wallet export
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedKeystore {
    pub crypto: EncryptedData,
    pub address: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: String,
    pub nonce: String,
    pub salt: String,
    pub iterations: u32,
}

impl EncryptedKeystore {
    /// Encrypt a private key with password and export keystore JSON
    pub fn encrypt(private_key: &PrivateKey, public_key: &PublicKey, password: &str) -> Self {
        let mut salt = [0u8; SALT_LEN];
        OsRng.fill_bytes(&mut salt);

        let mut nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce);

        let mut key = [0u8; 32];
        pbkdf2_hmac(password.as_bytes(), &salt, PBKDF2_ITERATIONS, Sha256::new(), &mut key);

        let aes_key = Key::from_slice(&key);
        let cipher = Aes256Gcm::new(aes_key);

        let nonce_slice = Nonce::from_slice(&nonce);
        let ciphertext = cipher.encrypt(nonce_slice, private_key.to_bytes().as_ref())
            .expect("Encryption failed");

        EncryptedKeystore {
            address: crate::wallet::wallet::derive_address(public_key),
            public_key: hex::encode(public_key.to_bytes()),
            crypto: EncryptedData {
                ciphertext: b64e(ciphertext),
                nonce: b64e(nonce),
                salt: b64e(salt),
                iterations: PBKDF2_ITERATIONS,
            }
        }
    }

    /// Decrypt a keystore JSON with password and return private key
    pub fn decrypt(&self, password: &str) -> Option<PrivateKey> {
        let salt = b64d(&self.crypto.salt).ok()?;
        let nonce = b64d(&self.crypto.nonce).ok()?;
        let ciphertext = b64d(&self.crypto.ciphertext).ok()?;

        let mut key = [0u8; 32];
        pbkdf2_hmac(password.as_bytes(), &salt, self.crypto.iterations, Sha256::new(), &mut key);

        let aes_key = Key::from_slice(&key);
        let cipher = Aes256Gcm::new(aes_key);
        let nonce = Nonce::from_slice(&nonce);

        let decrypted_bytes = cipher.decrypt(nonce, ciphertext.as_ref()).ok()?;
        use k256::SecretKey;
        let secret = SecretKey::from_bytes(&decrypted_bytes).ok()?;

        Some(PrivateKey { inner: secret })
    }
}

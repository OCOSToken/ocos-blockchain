//! # wallet::mnemonic
//!
//! Provides deterministic wallet key generation from BIP39 mnemonic phrases.
//! Fully compatible with OCOS wallet structure and private key usage.
//!
//! Uses industry-standard BIP39 + optional password (seed entropy hardening).

use bip39::{Mnemonic, Language, Seed};
use crate::crypto::keypair::{PrivateKey, PublicKey};
use hmac::{Hmac, Mac};
use sha2::Sha512;

/// Represents a wallet mnemonic and derived keypair.
pub struct MnemonicWallet {
    pub mnemonic_phrase: String,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

impl MnemonicWallet {
    /// Generates a new 12-word mnemonic and derives the corresponding keypair.
    pub fn generate_new(password: Option<&str>) -> Self {
        let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();
        Self::from_mnemonic(&mnemonic.phrase(), password)
    }

    /// Restores wallet from existing mnemonic phrase + optional password.
    pub fn from_mnemonic(mnemonic_phrase: &str, password: Option<&str>) -> Self {
        let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English)
            .expect("Invalid mnemonic");

        let seed = Seed::new(&mnemonic, password.unwrap_or(""));
        let private_key = derive_private_key_from_seed(seed.as_bytes());
        let public_key = private_key.public_key();

        MnemonicWallet {
            mnemonic_phrase: mnemonic.phrase().to_string(),
            private_key,
            public_key,
        }
    }
}

/// Derives a 32-byte private key from a 64-byte seed using HMAC-SHA512
fn derive_private_key_from_seed(seed: &[u8]) -> PrivateKey {
    let mut hmac = Hmac::<Sha512>::new_from_slice(b"OCOS HD Key").unwrap();
    hmac.update(seed);
    let result = hmac.finalize().into_bytes();
    let key_bytes = &result[..32];

    use k256::SecretKey;
    let secret = SecretKey::from_bytes(key_bytes.into()).expect("Valid secp256k1 key");
    PrivateKey { inner: secret }
}

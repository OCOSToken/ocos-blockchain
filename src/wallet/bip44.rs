//! # wallet::bip44
//!
//! Implements BIP44-compliant hierarchical deterministic wallet structure
//! for OCOS. Allows deterministic generation of multiple accounts from a
//! single mnemonic seed.
//!
//! Path structure: m / 44' / coin_type' / account' / change / index

use bip39::{Mnemonic, Language, Seed};
use bip32::{Mnemonic as Bip32Mnemonic, DerivationPath, XPrv};
use crate::crypto::keypair::{PrivateKey, PublicKey};
use sha2::{Sha256, Digest};

/// OCOS coin type according to SLIP-0044.
/// You may apply to get a real index; we'll use 4747 as placeholder.
const OCOS_COIN_TYPE: u32 = 4747;

/// Represents a derived HD wallet account from a specific BIP44 path.
pub struct Bip44Account {
    pub derivation_path: String,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub address: String,
}

impl Bip44Account {
    /// Derives a wallet at the given BIP44 path: (account, change, index)
    pub fn from_mnemonic(
        phrase: &str,
        password: Option<&str>,
        account: u32,
        change: u32,
        index: u32,
    ) -> Self {
        let mnemonic = Bip32Mnemonic::new(phrase.parse::<Mnemonic>().unwrap(), Language::English);
        let seed = Seed::new(mnemonic.as_ref(), password.unwrap_or(""));
        let master_key = XPrv::new(seed.as_bytes()).unwrap();

        let path = format!("m/44'/{}'/{}'/{}/{}", OCOS_COIN_TYPE, account, change, index);
        let derivation_path = path.parse::<DerivationPath>().expect("Valid derivation path");

        let derived_xprv = master_key.derive_path(&derivation_path).unwrap();
        let secret_bytes = derived_xprv.to_bytes();

        use k256::SecretKey;
        let secret = SecretKey::from_bytes(secret_bytes).expect("Valid secp256k1 key");
        let private_key = PrivateKey { inner: secret };
        let public_key = private_key.public_key();
        let address = derive_address(&public_key);

        Bip44Account {
            derivation_path: path,
            private_key,
            public_key,
            address,
        }
    }
}

/// Simple OCOS address derivation (same as in `wallet.rs`)
fn derive_address(pubkey: &PublicKey) -> String {
    let sha256 = Sha256::digest(pubkey.to_bytes());
    let hash160 = ripemd::Ripemd160::digest(&sha256);
    format!("OCOS{}", hex::encode(hash160))
}

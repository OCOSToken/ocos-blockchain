//! # wallet::multisig
//!
//! Provides M-of-N multi-signature wallet functionality using ECDSA (secp256k1).
//!
//! Enables collective control of assets or operations requiring multiple parties' approval.

use crate::crypto::keypair::PublicKey;
use crate::crypto::signature::{Signature, sign_message, verify_signature};
use crate::crypto::keypair::PrivateKey;

/// Represents a multi-signature wallet configuration (M-of-N).
#[derive(Debug, Clone)]
pub struct MultisigWallet {
    pub required_signatures: usize,
    pub public_keys: Vec<PublicKey>,
}

/// Holds collected partial signatures for a specific message.
#[derive(Debug)]
pub struct MultisigSignature {
    pub message: Vec<u8>,
    pub signatures: Vec<(PublicKey, Signature)>,
}

impl MultisigWallet {
    /// Creates a new M-of-N multisig wallet.
    pub fn new(required_signatures: usize, public_keys: Vec<PublicKey>) -> Self {
        assert!(
            required_signatures <= public_keys.len(),
            "Required signatures cannot exceed total public keys"
        );
        MultisigWallet {
            required_signatures,
            public_keys,
        }
    }

    /// Verifies that a given `MultisigSignature` contains M valid signatures.
    pub fn verify(&self, multisig_sig: &MultisigSignature) -> bool {
        let mut valid_count = 0;

        for (pk, sig) in &multisig_sig.signatures {
            if self.public_keys.contains(pk)
                && verify_signature(&multisig_sig.message, sig, pk)
            {
                valid_count += 1;
            }
        }

        valid_count >= self.required_signatures
    }
}

/// Signs a message as a multisig participant.
pub fn sign_multisig_participant(
    message: &[u8],
    signer_private: &PrivateKey,
) -> (PublicKey, Signature) {
    let signature = sign_message(message, signer_private);
    let public_key = signer_private.public_key();
    (public_key, signature)
}

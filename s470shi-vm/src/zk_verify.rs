// src/zk_verify.rs – S470SHI VM Zero-Knowledge Proof Verifier

use regex::Regex;

/// Verifies whether a given zk-proof string is valid
pub fn zk_verify(proof: &str) -> bool {
    // Example: zk-0xabcdef... (66-char hex)
    let pattern = Regex::new(r"^zk-0x[a-fA-F0-9]{60}$").unwrap();
    pattern.is_match(proof)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_proof() {
        let valid = "zk-0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab";
        assert!(zk_verify(valid));
    }

    #[test]
    fn test_invalid_proof_short() {
        let invalid = "zk-0x123456";
        assert!(!zk_verify(invalid));
    }

    #[test]
    fn test_invalid_proof_no_prefix() {
        let invalid = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab";
        assert!(!zk_verify(invalid));
    }

    #[test]
    fn test_invalid_proof_non_hex() {
        let invalid = "zk-0xXYZ4567890abcdef1234567890abcdef1234567890abcdef1234567890ab";
        assert!(!zk_verify(invalid));
    }
}

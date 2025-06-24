use crate::transaction::Transaction;

/// The TransactionValidator struct is responsible for validating
/// pending transactions before they are accepted into the mempool
/// or committed into a block.
///
/// This includes:
/// - Structural validation (fields not empty, amount > 0)
/// - Signature presence (and later, signature verification)
/// - Optional: balance check, nonce validation, fee policy
pub struct TransactionValidator;

impl TransactionValidator {
    /// Checks if a transaction is structurally valid
    pub fn is_structurally_valid(tx: &Transaction) -> Result<(), String> {
        if tx.from.trim().is_empty() {
            return Err("Sender address is empty.".into());
        }
        if tx.to.trim().is_empty() {
            return Err("Recipient address is empty.".into());
        }
        if tx.amount == 0 {
            return Err("Transaction amount must be greater than zero.".into());
        }
        if tx.message.len() > 512 {
            return Err("Transaction message is too long.".into());
        }
        Ok(())
    }

    /// Checks whether the transaction includes a signature
    pub fn has_signature(tx: &Transaction) -> Result<(), String> {
        if tx.signature.is_none() {
            return Err("Transaction is not signed.".into());
        }
        Ok(())
    }

    /// Overall validator entrypoint
    pub fn validate(tx: &Transaction) -> Result<(), String> {
        Self::is_structurally_valid(tx)?;
        Self::has_signature(tx)?;
        // Future: Self::verify_signature(tx)?;
        // Future: Self::check_balance(tx)?;
        Ok(())
    }
}

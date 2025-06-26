// wasm_executor/exec.rs – S470SHI VM Runtime Executor

use crate::parser::{IntentParser, ParsedIntent, IntentType};
use crate::stream_tape::{StreamTape, Transaction};
use crate::zk_verify::zk_verify;
use crate::reputation::{ReputationEngine, ExecTier};
use chrono::Utc;

/// Executes a parsed intent on the S470SHI Virtual Machine
pub fn execute_intent(
    source_code: &str,
    from: &str,
    reputation_engine: &mut ReputationEngine,
    stream: &mut StreamTape,
    current_block: u64,
) -> Result<String, String> {
    let parsed: ParsedIntent = IntentParser::parse(source_code);

    // Assign execution tier
    let tier = reputation_engine.get_exec_tier(from);
    match tier {
        ExecTier::Limited => return Err("Execution denied: insufficient reputation".into()),
        _ => {},
    }

    // Optional zk validation (if tag present)
    if let Some(proof) = parsed.params.get("zk_hash") {
        if !zk_verify(proof) {
            return Err("Invalid zk-proof submitted.".into());
        }
    }

    // Simulate transaction creation (mock execution output)
    let tx = Transaction {
        from: from.to_string(),
        to: parsed.params.get("to").unwrap_or(&"dao.treasury".to_string()).to_string(),
        amount: parsed.params.get("amount")
            .and_then(|a| a.parse::<f64>().ok())
            .unwrap_or(0.0),
        memo: parsed.params.get("memo").unwrap_or(&"".to_string()).to_string(),
        zk_hash: parsed.params.get("zk_hash").unwrap_or(&"".to_string()).to_string(),
        timestamp: Utc::now(),
        tag: parsed.params.get("tag").cloned(),
    };

    // Commit to stream
    stream.commit(tx)?;

    // Optionally update reputation if DAO event
    let vote_cast = matches!(parsed.intent, IntentType::Governance);
    reputation_engine.update_reputation(from, vote_cast, current_block);

    Ok(format!("✅ Intent executed successfully as {:?}", parsed.intent))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reputation::ReputationEngine;

    #[test]
    fn test_execute_governance_intent() {
        let code = r#"
            contract DAOValidator {
                dao.vote "Enable zkVault"
                memo: bootstrap
                amount: 0.047
                zk_hash: zk-0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab
                tag: genesis
            }
        "#;

        let mut rep = ReputationEngine::new();
        let mut stream = StreamTape::new();
        let result = execute_intent(code, "0xAlice", &mut rep, &mut stream, 1001);
        assert!(result.is_ok());
        assert_eq!(stream.log.len(), 1);
    }

    #[test]
    fn test_execute_rejected_for_low_reputation() {
        let mut rep = ReputationEngine::new();
        let mut stream = StreamTape::new();

        // Manually degrade reputation
        rep.update_reputation("0xBob", false, 999);
        for _ in 0..20 {
            rep.update_reputation("0xBob", false, 999);
        }

        let result = execute_intent("dao.vote \"Ban bad actor\"", "0xBob", &mut rep, &mut stream, 1002);
        assert!(result.is_err());
    }
}

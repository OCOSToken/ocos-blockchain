// tests/dao_exec.rs – S470SHI VM DAO Governance Execution Tests

use s470shi_vm::exec::execute_intent;
use s470shi_vm::parser::IntentType;
use s470shi_vm::stream_tape::StreamTape;
use s470shi_vm::reputation::ReputationEngine;
use chrono::Utc;

#[test]
fn test_valid_dao_vote_intent_executes() {
    let source = r#"
        contract DAOValidator {
            dao.vote "Upgrade zkModule"
            memo: initiate upgrade
            amount: 0.047
            zk_hash: zk-0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd
            tag: dao-governance
        }
    "#;

    let mut rep = ReputationEngine::new();
    let mut stream = StreamTape::new();
    let result = execute_intent(source, "0xValidator01", &mut rep, &mut stream, 12000);

    assert!(result.is_ok());
    assert_eq!(stream.log.len(), 1);
    let tx = &stream.log[0];
    assert_eq!(tx.tag.as_ref().unwrap(), "dao-governance");
    assert_eq!(tx.amount, 0.047);
}

#[test]
fn test_vote_requires_valid_zk_hash() {
    let source = r#"
        contract DAOValidator {
            dao.vote "Change Policy"
            memo: invalid zk
            amount: 0.1
            zk_hash: zk-xyz-invalid
            tag: fail-case
        }
    "#;

    let mut rep = ReputationEngine::new();
    let mut stream = StreamTape::new();
    let result = execute_intent(source, "0xAlice", &mut rep, &mut stream, 12010);

    assert!(result.is_err());
    assert_eq!(stream.log.len(), 0);
}

#[test]
fn test_low_reputation_rejection() {
    let source = "dao.vote \"Remove Validator\"";
    let mut rep = ReputationEngine::new();
    let mut stream = StreamTape::new();

    for _ in 0..25 {
        rep.update_reputation("0xLowRep", false, 11500);
    }

    let result = execute_intent(source, "0xLowRep", &mut rep, &mut stream, 12020);
    assert!(result.is_err());
    assert_eq!(stream.log.len(), 0);
}

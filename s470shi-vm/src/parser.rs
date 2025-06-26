// src/parser.rs – S470SHI Virtual Machine Intent Parser

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum IntentType {
    Governance,
    VaultLogic,
    ZkTransfer,
    StreamNote,
    Standard,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ParsedIntent {
    pub intent: IntentType,
    pub params: HashMap<String, String>,
    pub raw: String,
}

pub struct IntentParser;

impl IntentParser {
    pub fn parse(source: &str) -> ParsedIntent {
        let mut params = HashMap::new();

        let intent = if source.contains("dao.vote") {
            IntentType::Governance
        } else if source.contains("vault") && source.contains("unlock_at") {
            IntentType::VaultLogic
        } else if source.contains("zk_transaction") {
            IntentType::ZkTransfer
        } else if source.contains("stream.log") {
            IntentType::StreamNote
        } else {
            IntentType::Standard
        };

        // Extract parameters (very simple prototype parser)
        for line in source.lines() {
            if let Some((k, v)) = line.split_once(":") {
                params.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        ParsedIntent {
            intent,
            params,
            raw: source.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_parsing() {
        let input = r#"
            contract DAOValidator {
                require identity.reputation > 47
                dao.vote "Activate zkSwap"
            }
        "#;
        let parsed = IntentParser::parse(input);
        assert_eq!(parsed.intent, IntentType::Governance);
        assert!(parsed.raw.contains("dao.vote"));
    }

    #[test]
    fn test_unknown_parsing() {
        let input = "function simpleTransfer() {}";
        let parsed = IntentParser::parse(input);
        assert_eq!(parsed.intent, IntentType::Standard);
    }
}

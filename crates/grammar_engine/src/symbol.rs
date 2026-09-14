use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(char),
    NonTerminal(String),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Terminal(c) => write!(f, "{c}"),
            Symbol::NonTerminal(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarError {
    ParseError(String),
    UndefinedNonTerminal(String),
    NotRegular {
        non_terminal: String,
        alternative: String,
        reason: String,
    },
    EmptyGrammar,
    DerivationTooLong,
    RegexConversionFailed(String),
}

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrammarError::ParseError(msg) => write!(f, "erro de sintaxe na gramática: {msg}"),
            GrammarError::UndefinedNonTerminal(name) => {
                write!(f, "não-terminal '{name}' usado mas nunca definido")
            }
            GrammarError::NotRegular {
                non_terminal,
                alternative,
                reason,
            } => write!(
                f,
                "produção '{non_terminal} -> {alternative}' não é regular: {reason}"
            ),
            GrammarError::EmptyGrammar => write!(f, "gramática vazia"),
            GrammarError::DerivationTooLong => write!(
                f,
                "derivação excedeu o limite de passos (gramática pode não ser produtiva)"
            ),
            GrammarError::RegexConversionFailed(msg) => {
                write!(f, "falha ao converter para expressão regular: {msg}")
            }
        }
    }
}

impl std::error::Error for GrammarError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_displays_as_its_char() {
        assert_eq!(Symbol::Terminal('a').to_string(), "a");
    }

    #[test]
    fn non_terminal_displays_as_its_name() {
        assert_eq!(Symbol::NonTerminal("S".to_string()).to_string(), "S");
    }

    #[test]
    fn symbols_are_usable_as_hashmap_keys() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(Symbol::NonTerminal("S".to_string()), 1);
        assert_eq!(map.get(&Symbol::NonTerminal("S".to_string())), Some(&1));
    }

    #[test]
    fn not_regular_error_message_is_readable() {
        let err = GrammarError::NotRegular {
            non_terminal: "S".to_string(),
            alternative: "AB".to_string(),
            reason: "não-terminal só pode aparecer como último símbolo".to_string(),
        };
        let message = err.to_string();
        assert!(message.contains('S'));
        assert!(message.contains("AB"));
    }
}

use crate::grammar::{Grammar, parse_grammar};

#[derive(Debug, Clone, Copy)]
pub struct ExampleGrammar {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
}

pub const EXAMPLE_SOURCES: [ExampleGrammar; 3] = [
    ExampleGrammar {
        name: "Exemplo do professor",
        description: "S -> aS | ab — o exemplo do slide da disciplina.",
        source: "S -> aS | ab",
    },
    ExampleGrammar {
        name: "Cadeia a-b-c",
        description: "Uma sequência de a's, seguida de b's, terminando em c.",
        source: "S -> aS | aA\nA -> bA | c",
    },
    ExampleGrammar {
        name: "Labirinto de bifurcações",
        description: "Três não-terminais interligados — pensado para o modo labirinto.",
        source: "S -> aA | bB\nA -> bA | aC\nB -> aB | bC\nC -> a",
    },
];

/// Parses the three built-in grammars. Panics only if one of the hardcoded
/// `source` strings above is itself malformed — a programmer error caught
/// immediately by this module's own tests, never a possibility from user
/// input (see Global Constraints in the plan this implements).
pub fn examples() -> Vec<(&'static ExampleGrammar, Grammar)> {
    EXAMPLE_SOURCES
        .iter()
        .map(|example| {
            let grammar = parse_grammar(example.source).unwrap_or_else(|err| {
                panic!(
                    "gramática de exemplo embutida '{}' é inválida: {err}",
                    example.name
                )
            });
            (example, grammar)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::derive_random;
    use crate::grammar::validate_regular;
    use crate::regex_conversion::to_regex;
    use regex::Regex;

    #[test]
    fn ships_exactly_three_examples() {
        assert_eq!(EXAMPLE_SOURCES.len(), 3);
    }

    #[test]
    fn every_example_parses_and_is_a_valid_regular_grammar() {
        for (example, grammar) in examples() {
            assert!(
                validate_regular(&grammar).is_ok(),
                "exemplo '{}' não é uma gramática regular válida",
                example.name
            );
        }
    }

    #[test]
    fn every_example_can_derive_a_sentence_matching_its_own_regex() {
        for (example, grammar) in examples() {
            let pattern_text = to_regex(&grammar).unwrap();
            let pattern = Regex::new(&format!("^{pattern_text}$")).unwrap();
            let derivation = derive_random(&grammar).unwrap();
            assert!(
                pattern.is_match(&derivation.sentence),
                "exemplo '{}': sentença '{}' não bate com {pattern_text}",
                example.name,
                derivation.sentence
            );
        }
    }
}

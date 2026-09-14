//! Motor de gramáticas regulares: parsing, derivação via pilha e conversão
//! para expressão regular. Ver docs/superpowers/specs/2026-09-14-grammar-quest-design.md.

mod derivation;
mod examples;
mod grammar;
mod regex_conversion;
mod stack;
mod symbol;

pub use derivation::{Derivation, DerivationState, DerivationStep, derive_random};
pub use examples::{EXAMPLE_SOURCES, ExampleGrammar, examples};
pub use grammar::{Grammar, parse_grammar, validate_regular};
pub use regex_conversion::to_regex;
pub use stack::Stack;
pub use symbol::{GrammarError, Symbol};

#[cfg(test)]
mod public_api_tests {
    use super::*;

    #[test]
    fn end_to_end_public_api_reproduces_the_pdf_example() {
        let grammar = parse_grammar("S -> aS | ab").expect("grammar text is valid");
        validate_regular(&grammar).expect("grammar is regular");
        let derivation = derive_random(&grammar).expect("grammar is productive");
        assert!(!derivation.sentence.is_empty());
        assert_eq!(to_regex(&grammar).expect("grammar converts"), "a*ab");
        assert_eq!(examples().len(), 3);
    }
}

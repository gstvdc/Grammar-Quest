use rand::Rng;

use crate::grammar::Grammar;
use crate::stack::Stack;
use crate::symbol::{GrammarError, Symbol};

const MAX_DERIVATION_STEPS: usize = 10_000;

#[derive(Debug, Clone)]
pub struct DerivationStep {
    pub non_terminal: String,
    pub production: Vec<Symbol>,
    pub stack_after: Vec<Symbol>,
    pub output_so_far: String,
}

#[derive(Debug, Clone)]
pub struct Derivation {
    pub steps: Vec<DerivationStep>,
    pub sentence: String,
}

/// Implements the professor's algorithm exactly: pick a production for the
/// current non-terminal, push it (leftmost symbol on top), then while the
/// stack isn't empty pop — terminals go to the output, non-terminals get
/// expanded again from step one.
pub fn derive_random(grammar: &Grammar) -> Result<Derivation, GrammarError> {
    let mut rng = rand::thread_rng();
    let mut stack = Stack::new();
    let mut output = String::new();
    let mut steps = Vec::new();

    stack.push_production(&[Symbol::NonTerminal(grammar.start.clone())]);

    let mut guard = 0usize;
    let mut pending_productions: Vec<(String, Vec<Symbol>, usize)> = Vec::new();

    while let Some(symbol) = stack.pop() {
        guard += 1;
        if guard > MAX_DERIVATION_STEPS {
            return Err(GrammarError::DerivationTooLong);
        }

        match symbol {
            Symbol::Terminal(c) => {
                output.push(c);
                // Decrement the counter for the top pending production
                if let Some((_, _, remaining)) = pending_productions.last_mut() {
                    *remaining -= 1;
                    if *remaining == 0 {
                        let (name, production, _) = pending_productions.pop().unwrap();
                        steps.push(DerivationStep {
                            non_terminal: name,
                            production,
                            stack_after: stack.snapshot_top_first(),
                            output_so_far: output.clone(),
                        });
                    }
                }
            }
            Symbol::NonTerminal(name) => {
                let alternatives = grammar
                    .alternatives(&name)
                    .ok_or_else(|| GrammarError::UndefinedNonTerminal(name.clone()))?;
                let choice = rng.gen_range(0..alternatives.len());
                let production = alternatives[choice].clone();
                let production_size = production.len();
                stack.push_production(&production);
                pending_productions.push((name, production, production_size));
            }
        }
    }

    Ok(Derivation {
        steps,
        sentence: output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::parse_grammar;
    use crate::symbol::GrammarError;
    use regex::Regex;

    #[test]
    fn derives_a_sentence_matching_the_pdf_example_language() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let pattern = Regex::new(r"^a*ab$").unwrap();
        for _ in 0..50 {
            let derivation = derive_random(&grammar).unwrap();
            assert!(
                pattern.is_match(&derivation.sentence),
                "sentença '{}' não bate com a*ab",
                derivation.sentence
            );
        }
    }

    #[test]
    fn records_one_step_per_non_terminal_expansion() {
        let grammar = parse_grammar("S -> a").unwrap();
        let derivation = derive_random(&grammar).unwrap();
        assert_eq!(derivation.steps.len(), 1);
        assert_eq!(derivation.sentence, "a");
        assert_eq!(derivation.steps[0].non_terminal, "S");
        assert_eq!(derivation.steps[0].output_so_far, "a");
        assert!(derivation.steps[0].stack_after.is_empty());
    }

    #[test]
    fn errors_out_instead_of_looping_forever_on_a_dead_end_grammar() {
        let grammar = parse_grammar("S -> aS").unwrap();
        let result = derive_random(&grammar);
        assert_eq!(result.unwrap_err(), GrammarError::DerivationTooLong);
    }
}

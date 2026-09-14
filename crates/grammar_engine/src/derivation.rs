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

#[derive(Debug, Clone)]
pub struct DerivationState {
    pub stack: Stack,
    pub output: String,
    pub steps: Vec<DerivationStep>,
}

impl DerivationState {
    pub fn new(grammar: &Grammar) -> Self {
        let mut stack = Stack::new();
        stack.push_production(&[Symbol::NonTerminal(grammar.start.clone())]);
        Self {
            stack,
            output: String::new(),
            steps: Vec::new(),
        }
    }

    pub fn current_non_terminal(&self) -> Option<String> {
        match self.stack.peek()? {
            Symbol::NonTerminal(name) => Some(name.clone()),
            Symbol::Terminal(_) => None,
        }
    }

    pub fn consume_terminals(&mut self) {
        while let Some(Symbol::Terminal(_)) = self.stack.peek() {
            if let Some(Symbol::Terminal(c)) = self.stack.pop() {
                self.output.push(c);
            }
        }
    }

    pub fn apply_choice(&mut self, grammar: &Grammar, choice: usize) -> Result<(), GrammarError> {
        self.consume_terminals();
        let Symbol::NonTerminal(name) = self.stack.pop().ok_or(GrammarError::DerivationTooLong)?
        else {
            return Ok(());
        };
        let alternatives = grammar
            .alternatives(&name)
            .ok_or_else(|| GrammarError::UndefinedNonTerminal(name.clone()))?;
        let production = alternatives
            .get(choice)
            .cloned()
            .ok_or_else(|| GrammarError::ParseError("escolha de produção inválida".to_string()))?;
        self.stack.push_production(&production);
        self.steps.push(DerivationStep {
            non_terminal: name,
            production,
            stack_after: self.stack.snapshot_top_first(),
            output_so_far: self.output.clone(),
        });
        self.consume_terminals();
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.stack.is_empty()
    }
}

/// Implements the professor's algorithm exactly: pick a production for the
/// current non-terminal, push it (leftmost symbol on top), then while the
/// stack isn't empty pop — terminals go to the output, non-terminals get
/// expanded again from step one.
pub fn derive_random(grammar: &Grammar) -> Result<Derivation, GrammarError> {
    let mut rng = rand::thread_rng();
    let mut state = DerivationState::new(grammar);
    let mut guard = 0usize;

    while !state.is_complete() {
        guard += 1;
        if guard > MAX_DERIVATION_STEPS {
            return Err(GrammarError::DerivationTooLong);
        }

        let nt = state
            .current_non_terminal()
            .ok_or(GrammarError::DerivationTooLong)?;
        let alternatives = grammar
            .alternatives(&nt)
            .ok_or_else(|| GrammarError::UndefinedNonTerminal(nt.clone()))?;
        let choice = rng.gen_range(0..alternatives.len());
        state.apply_choice(grammar, choice)?;
    }

    Ok(Derivation {
        steps: state.steps,
        sentence: state.output,
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
        assert_eq!(derivation.steps[0].output_so_far, "");
        assert_eq!(derivation.steps[0].stack_after, vec![Symbol::Terminal('a')]);
    }

    #[test]
    fn errors_out_instead_of_looping_forever_on_a_dead_end_grammar() {
        let grammar = parse_grammar("S -> aS").unwrap();
        let result = derive_random(&grammar);
        assert_eq!(result.unwrap_err(), GrammarError::DerivationTooLong);
    }

    #[test]
    fn records_all_non_terminal_expansions_at_all_depths() {
        let grammar = parse_grammar("S -> aA\nA -> a").unwrap();
        let derivation = derive_random(&grammar).unwrap();
        assert_eq!(derivation.sentence, "aa");
        assert_eq!(derivation.steps.len(), 2);
        assert_eq!(derivation.steps[0].non_terminal, "S");
        assert_eq!(derivation.steps[0].output_so_far, "");
        assert_eq!(derivation.steps[1].non_terminal, "A");
        assert_eq!(derivation.steps[1].output_so_far, "a");
    }

    #[test]
    fn derivation_state_manages_controlled_choices_and_completion() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let mut state = DerivationState::new(&grammar);

        assert_eq!(state.current_non_terminal().as_deref(), Some("S"));
        assert!(!state.is_complete());
        assert_eq!(state.output, "");

        // Apply choice 0: S -> aS
        state.apply_choice(&grammar, 0).unwrap();
        assert_eq!(state.output, "a");
        assert_eq!(state.current_non_terminal().as_deref(), Some("S"));
        assert_eq!(state.steps.len(), 1);
        assert_eq!(state.steps[0].non_terminal, "S");
        assert_eq!(
            state.steps[0].production,
            vec![Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]
        );

        // Apply choice 1: S -> ab
        state.apply_choice(&grammar, 1).unwrap();
        assert_eq!(state.output, "aab");
        assert!(state.is_complete());
        assert_eq!(state.current_non_terminal(), None);
        assert_eq!(state.steps.len(), 2);
    }
}

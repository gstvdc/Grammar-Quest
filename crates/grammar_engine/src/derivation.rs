use std::collections::HashMap;

use rand::{Rng, seq::SliceRandom};

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

/// One observable moment of the professor's stack algorithm, in the exact
/// order it happens, so the UI can replay it without re-deriving instants
/// from `DerivationStep` snapshots (see docs/audits/2026-09-15-project-audit.md).
#[derive(Debug, Clone, PartialEq)]
pub enum DerivationEvent {
    ProductionChosen {
        non_terminal: String,
        alternative_index: usize,
        production: Vec<Symbol>,
    },
    Pushed {
        stack_after: Vec<Symbol>,
    },
    TerminalPopped {
        terminal: char,
        output_so_far: String,
    },
    Completed {
        sentence: String,
    },
}

#[derive(Debug, Clone)]
pub struct Derivation {
    pub steps: Vec<DerivationStep>,
    pub events: Vec<DerivationEvent>,
    pub sentence: String,
}

#[derive(Debug, Clone)]
pub struct DerivationState {
    pub stack: Stack,
    pub output: String,
    pub steps: Vec<DerivationStep>,
    pub events: Vec<DerivationEvent>,
}

impl DerivationState {
    pub fn new(grammar: &Grammar) -> Self {
        let mut stack = Stack::new();
        stack.push_production(&[Symbol::NonTerminal(grammar.start.clone())]);
        Self {
            stack,
            output: String::new(),
            steps: Vec::new(),
            events: Vec::new(),
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
                self.events.push(DerivationEvent::TerminalPopped {
                    terminal: c,
                    output_so_far: self.output.clone(),
                });
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
        self.events.push(DerivationEvent::ProductionChosen {
            non_terminal: name.clone(),
            alternative_index: choice,
            production: production.clone(),
        });
        self.stack.push_production(&production);
        self.events.push(DerivationEvent::Pushed {
            stack_after: self.stack.snapshot_top_first(),
        });
        self.steps.push(DerivationStep {
            non_terminal: name,
            production,
            stack_after: self.stack.snapshot_top_first(),
            output_so_far: self.output.clone(),
        });
        self.consume_terminals();
        if self.is_complete() {
            self.events.push(DerivationEvent::Completed {
                sentence: self.output.clone(),
            });
        }
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
        events: state.events,
        sentence: state.output,
    })
}

/// Generates a terminating derivation whose number of non-terminal
/// expansions is inside the inclusive range. The search is bounded by the
/// requested maximum and selects randomly among only paths known to finish.
pub fn derive_random_in_step_range(
    grammar: &Grammar,
    min_steps: usize,
    max_steps: usize,
) -> Result<Derivation, GrammarError> {
    if min_steps == 0 || min_steps > max_steps {
        return Err(GrammarError::NoDerivationInStepRange {
            min: min_steps,
            max: max_steps,
        });
    }

    let mut memo = HashMap::new();
    let possible_lengths: Vec<usize> = (min_steps..=max_steps)
        .filter(|steps| can_terminate_in_steps(grammar, &grammar.start, *steps, &mut memo))
        .collect();
    let mut rng = rand::thread_rng();
    let Some(&remaining) = possible_lengths.choose(&mut rng) else {
        return Err(GrammarError::NoDerivationInStepRange {
            min: min_steps,
            max: max_steps,
        });
    };

    let mut state = DerivationState::new(grammar);
    let mut steps_left = remaining;
    while !state.is_complete() {
        let non_terminal = state
            .current_non_terminal()
            .ok_or(GrammarError::DerivationTooLong)?;
        let alternatives = grammar
            .alternatives(&non_terminal)
            .ok_or_else(|| GrammarError::UndefinedNonTerminal(non_terminal.clone()))?;
        let valid_choices: Vec<usize> = alternatives
            .iter()
            .enumerate()
            .filter_map(|(index, production)| {
                let finishes_in_range = match production_ends_in(production) {
                    Some(next) => can_terminate_in_steps(
                        grammar,
                        next,
                        steps_left.saturating_sub(1),
                        &mut memo,
                    ),
                    None => steps_left == 1,
                };
                finishes_in_range.then_some(index)
            })
            .collect();
        let choice =
            *valid_choices
                .choose(&mut rng)
                .ok_or(GrammarError::NoDerivationInStepRange {
                    min: min_steps,
                    max: max_steps,
                })?;
        state.apply_choice(grammar, choice)?;
        steps_left = steps_left.saturating_sub(1);
    }

    Ok(Derivation {
        steps: state.steps,
        events: state.events,
        sentence: state.output,
    })
}

fn can_terminate_in_steps(
    grammar: &Grammar,
    non_terminal: &str,
    steps: usize,
    memo: &mut HashMap<(String, usize), bool>,
) -> bool {
    if steps == 0 {
        return false;
    }
    let key = (non_terminal.to_owned(), steps);
    if let Some(result) = memo.get(&key) {
        return *result;
    }
    let result = grammar
        .alternatives(non_terminal)
        .is_some_and(|alternatives| {
            alternatives
                .iter()
                .any(|production| match production_ends_in(production) {
                    Some(next) => can_terminate_in_steps(grammar, next, steps - 1, memo),
                    None => steps == 1,
                })
        });
    memo.insert(key, result);
    result
}

fn production_ends_in(production: &[crate::Symbol]) -> Option<&str> {
    match production.last() {
        Some(crate::Symbol::NonTerminal(name)) => Some(name),
        _ => None,
    }
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
    fn derives_a_random_terminating_route_within_the_requested_step_range() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let derivation = derive_random_in_step_range(&grammar, 4, 6).unwrap();

        assert!((4..=6).contains(&derivation.steps.len()));
        assert!(derivation.sentence.ends_with('b'));
    }

    #[test]
    fn reports_when_a_step_range_cannot_terminate_in_the_grammar() {
        let grammar = parse_grammar("S -> aA\nA -> b").unwrap();

        assert_eq!(
            derive_random_in_step_range(&grammar, 3, 5).unwrap_err(),
            GrammarError::NoDerivationInStepRange { min: 3, max: 5 }
        );
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
    fn records_a_full_event_trace_matching_the_pdf_fixture() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let mut state = DerivationState::new(&grammar);

        state.apply_choice(&grammar, 0).unwrap();
        state.apply_choice(&grammar, 0).unwrap();
        state.apply_choice(&grammar, 1).unwrap();

        assert!(state.is_complete());
        assert_eq!(state.output, "aaab");

        let chosen: Vec<usize> = state
            .events
            .iter()
            .filter_map(|event| match event {
                DerivationEvent::ProductionChosen {
                    alternative_index, ..
                } => Some(*alternative_index),
                _ => None,
            })
            .collect();
        assert_eq!(chosen, vec![0, 0, 1]);

        let popped: Vec<char> = state
            .events
            .iter()
            .filter_map(|event| match event {
                DerivationEvent::TerminalPopped { terminal, .. } => Some(*terminal),
                _ => None,
            })
            .collect();
        assert_eq!(popped, vec!['a', 'a', 'a', 'b']);

        let pushed_stacks: Vec<Vec<Symbol>> = state
            .events
            .iter()
            .filter_map(|event| match event {
                DerivationEvent::Pushed { stack_after } => Some(stack_after.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(
            pushed_stacks[0],
            vec![Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]
        );

        match state.events.last() {
            Some(DerivationEvent::Completed { sentence }) => assert_eq!(sentence, "aaab"),
            other => panic!("esperava evento Completed no final, veio {other:?}"),
        }
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

    #[test]
    fn derives_a_left_linear_numeric_grammar_with_the_stack() {
        let grammar = parse_grammar("S -> S1 | S2 | S0 | ε").unwrap();
        let mut state = DerivationState::new(&grammar);

        state.apply_choice(&grammar, 0).unwrap();
        state.apply_choice(&grammar, 1).unwrap();
        state.apply_choice(&grammar, 3).unwrap();

        assert!(state.is_complete());
        assert_eq!(state.output, "21");
    }
}

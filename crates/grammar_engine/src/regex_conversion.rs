use std::collections::HashMap;

use crate::grammar::Grammar;
use crate::symbol::{GrammarError, Symbol};

#[derive(Debug, Clone)]
struct Term {
    coeff: String,
    target: Option<String>,
}

type Equation = Vec<Term>;

/// Converts a regular grammar into an equivalent regular expression using
/// the classic non-terminal elimination method (Arden's rule applied one
/// non-terminal at a time: `X = aX + b  =>  X = a*b`). Output uses
/// standard regex syntax (see Global Constraints in the plan this
/// implements) so it can be fed straight into the `regex` crate.
pub fn to_regex(grammar: &Grammar) -> Result<String, GrammarError> {
    let mut equations: HashMap<String, Equation> = grammar
        .non_terminals
        .iter()
        .map(|nt| (nt.clone(), build_equation(grammar, nt)))
        .collect();

    let non_start: Vec<String> = grammar
        .non_terminals
        .iter()
        .filter(|nt| **nt != grammar.start)
        .cloned()
        .collect();

    for eliminated in non_start {
        let equation = equations.remove(&eliminated).ok_or_else(|| {
            GrammarError::RegexConversionFailed(format!(
                "equação de '{eliminated}' não encontrada durante a eliminação"
            ))
        })?;
        let resolved = resolve_self_reference(&eliminated, equation);

        for equation in equations.values_mut() {
            substitute(equation, &eliminated, &resolved);
        }
    }

    let start_equation = equations.remove(&grammar.start).ok_or_else(|| {
        GrammarError::RegexConversionFailed("equação do símbolo inicial não encontrada".to_string())
    })?;
    let final_terms = resolve_self_reference(&grammar.start, start_equation);

    let mut pieces = Vec::with_capacity(final_terms.len());
    for term in final_terms {
        if term.target.is_some() {
            return Err(GrammarError::RegexConversionFailed(
                "gramática tem não-terminais que não puderam ser eliminados".to_string(),
            ));
        }
        pieces.push(term.coeff);
    }

    Ok(regex_union(&pieces))
}

fn build_equation(grammar: &Grammar, non_terminal: &str) -> Equation {
    grammar
        .alternatives(non_terminal)
        .into_iter()
        .flatten()
        .map(|alt| alternative_to_term(alt))
        .collect()
}

fn alternative_to_term(alt: &[Symbol]) -> Term {
    let mut coeff = String::new();
    let mut target = None;
    for symbol in alt {
        match symbol {
            Symbol::Terminal(c) => coeff.push(*c),
            Symbol::NonTerminal(name) => target = Some(name.clone()),
        }
    }
    Term { coeff, target }
}

/// Applies Arden's rule to remove `non_terminal` from its own equation,
/// distributing the resulting `alpha*` prefix over every remaining term.
fn resolve_self_reference(non_terminal: &str, equation: Equation) -> Equation {
    let (self_terms, mut other_terms): (Vec<Term>, Vec<Term>) = equation
        .into_iter()
        .partition(|term| term.target.as_deref() == Some(non_terminal));

    if self_terms.is_empty() {
        return other_terms;
    }

    let alpha = regex_union(&self_terms.into_iter().map(|t| t.coeff).collect::<Vec<_>>());
    let alpha_star = kleene_star(&alpha);

    for term in &mut other_terms {
        term.coeff = format!("{alpha_star}{}", term.coeff);
    }

    other_terms
}

fn substitute(equation: &mut Equation, eliminated: &str, resolved: &Equation) {
    let mut new_terms = Vec::new();
    for term in equation.drain(..) {
        if term.target.as_deref() == Some(eliminated) {
            for resolved_term in resolved {
                new_terms.push(Term {
                    coeff: format!("{}{}", term.coeff, resolved_term.coeff),
                    target: resolved_term.target.clone(),
                });
            }
        } else {
            new_terms.push(term);
        }
    }
    *equation = new_terms;
}

fn regex_union(parts: &[String]) -> String {
    match parts.len() {
        0 => String::new(),
        1 => parts[0].clone(),
        _ => format!("({})", parts.join("|")),
    }
}

fn kleene_star(alpha: &str) -> String {
    if alpha.is_empty() {
        String::new()
    } else if alpha.chars().count() == 1 {
        format!("{alpha}*")
    } else {
        format!("({alpha})*")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::derive_random;
    use crate::grammar::parse_grammar;
    use regex::Regex;

    #[test]
    fn matches_the_exact_pdf_example_output() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        assert_eq!(to_regex(&grammar).unwrap(), "a*ab");
    }

    #[test]
    fn handles_a_chain_of_two_non_terminals() {
        let grammar = parse_grammar("S -> aA\nA -> bA | c").unwrap();
        assert_eq!(to_regex(&grammar).unwrap(), "ab*c");
    }

    #[test]
    fn unions_multiple_self_referencing_alternatives_before_starring() {
        let grammar = parse_grammar("S -> aS | bS | c").unwrap();
        let pattern_text = to_regex(&grammar).unwrap();
        let pattern = Regex::new(&format!("^{pattern_text}$")).unwrap();
        assert!(pattern.is_match("c"));
        assert!(pattern.is_match("abababc"));
        assert!(!pattern.is_match("d"));
    }

    #[test]
    fn generated_sentences_always_match_the_derived_regex() {
        let grammar = parse_grammar("S -> aA | bB\nA -> bA | aC\nB -> aB | bC\nC -> a").unwrap();
        let pattern_text = to_regex(&grammar).unwrap();
        let pattern = Regex::new(&format!("^{pattern_text}$")).unwrap();
        for _ in 0..100 {
            let derivation = derive_random(&grammar).unwrap();
            assert!(
                pattern.is_match(&derivation.sentence),
                "sentença '{}' não bate com {pattern_text}",
                derivation.sentence
            );
        }
    }
}

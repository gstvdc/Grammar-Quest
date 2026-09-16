use std::collections::HashMap;

use crate::grammar::Grammar;
use crate::symbol::{GrammarError, Symbol};

#[derive(Debug, Clone)]
struct Term {
    coeff: String,
    target: Option<String>,
}

type Equation = Vec<Term>;

/// One non-terminal eliminated via Arden's rule, in elimination order, so
/// the UI can reproduce the professor's equation-by-equation walkthrough
/// (see docs/audits/2026-09-15-project-audit.md) instead of only the final
/// regex.
#[derive(Debug, Clone, PartialEq)]
pub struct EliminationStep {
    pub eliminated: String,
    pub resolved_equation: String,
    pub remaining_equations: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegexTrace {
    pub initial_equations: Vec<(String, String)>,
    pub eliminations: Vec<EliminationStep>,
    pub final_expression: String,
}

/// Converts a regular grammar into an equivalent regular expression using
/// the classic non-terminal elimination method (Arden's rule applied one
/// non-terminal at a time: `X = aX + b  =>  X = a*b`). Output uses
/// standard regex syntax (see Global Constraints in the plan this
/// implements) so it can be fed straight into the `regex` crate.
pub fn to_regex(grammar: &Grammar) -> Result<String, GrammarError> {
    Ok(to_regex_trace(grammar)?.final_expression)
}

/// Same conversion as [`to_regex`], but keeps every intermediate equation so
/// callers can display the derivation the PDF walks through by hand, not
/// just its final answer.
pub fn to_regex_trace(grammar: &Grammar) -> Result<RegexTrace, GrammarError> {
    let mut equations: HashMap<String, Equation> = grammar
        .non_terminals
        .iter()
        .map(|nt| (nt.clone(), build_equation(grammar, nt)))
        .collect();

    let initial_equations = ordered_display(grammar, &equations);

    let mut elimination_order: Vec<String> = grammar
        .non_terminals
        .iter()
        .filter(|nt| **nt != grammar.start)
        .cloned()
        .collect();
    elimination_order.push(grammar.start.clone());

    let mut eliminations = Vec::with_capacity(elimination_order.len());
    let mut final_terms = None;

    for eliminated in elimination_order {
        let equation = equations.remove(&eliminated).ok_or_else(|| {
            GrammarError::RegexConversionFailed(format!(
                "equação de '{eliminated}' não encontrada durante a eliminação"
            ))
        })?;
        let resolved = resolve_self_reference(&eliminated, equation);
        let resolved_equation = format_equation(&eliminated, &resolved);

        for equation in equations.values_mut() {
            substitute(equation, &eliminated, &resolved);
        }

        if equations.is_empty() {
            final_terms = Some(resolved.clone());
        }

        eliminations.push(EliminationStep {
            eliminated,
            resolved_equation,
            remaining_equations: ordered_display(grammar, &equations),
        });
    }

    let final_terms = final_terms.ok_or_else(|| {
        GrammarError::RegexConversionFailed("equação do símbolo inicial não encontrada".to_string())
    })?;

    if final_terms.is_empty() {
        return Err(GrammarError::RegexConversionFailed(
            "gramática não produz nenhuma sentença".to_string(),
        ));
    }

    let mut pieces = Vec::with_capacity(final_terms.len());
    for term in final_terms {
        if term.target.is_some() {
            return Err(GrammarError::RegexConversionFailed(
                "gramática tem não-terminais que não puderam ser eliminados".to_string(),
            ));
        }
        pieces.push(term.coeff);
    }

    Ok(RegexTrace {
        initial_equations,
        eliminations,
        final_expression: regex_union(&pieces),
    })
}

/// Renders `nt`'s equations in the grammar's declaration order (stable and
/// human-readable), skipping any already eliminated.
fn ordered_display(
    grammar: &Grammar,
    equations: &HashMap<String, Equation>,
) -> Vec<(String, String)> {
    grammar
        .non_terminals
        .iter()
        .filter_map(|nt| {
            equations
                .get(nt)
                .map(|eq| (nt.clone(), format_equation(nt, eq)))
        })
        .collect()
}

/// `S=aS+ab` style rendering matching the PDF's own equation notation
/// (`+` for union), distinct from the `|`-based regex output.
fn format_equation(non_terminal: &str, equation: &Equation) -> String {
    let terms: Vec<String> = equation.iter().map(format_term).collect();
    format!("{non_terminal}={}", terms.join("+"))
}

fn format_term(term: &Term) -> String {
    match &term.target {
        Some(target) => format!("{}{target}", term.coeff),
        None => term.coeff.clone(),
    }
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
    } else if alpha.chars().count() == 1 || (alpha.starts_with('(') && alpha.ends_with(')')) {
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
    use crate::symbol::GrammarError;
    use regex::Regex;

    #[test]
    fn traces_the_pdf_fixture_equation_and_elimination() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let trace = to_regex_trace(&grammar).unwrap();

        assert_eq!(
            trace.initial_equations,
            vec![("S".to_string(), "S=aS+ab".to_string())]
        );
        assert_eq!(trace.eliminations.len(), 1);
        assert_eq!(trace.eliminations[0].eliminated, "S");
        assert_eq!(trace.eliminations[0].resolved_equation, "S=a*ab");
        assert_eq!(trace.final_expression, "a*ab");
    }

    #[test]
    fn traces_a_chain_elimination_before_the_start_symbol() {
        let grammar = parse_grammar("S -> aA\nA -> bA | c").unwrap();
        let trace = to_regex_trace(&grammar).unwrap();

        assert_eq!(trace.eliminations.len(), 2);
        assert_eq!(trace.eliminations[0].eliminated, "A");
        assert_eq!(trace.eliminations[0].resolved_equation, "A=b*c");
        assert_eq!(trace.eliminations[1].eliminated, "S");
        assert_eq!(trace.eliminations[1].resolved_equation, "S=ab*c");
        assert_eq!(trace.final_expression, "ab*c");
    }

    #[test]
    fn converts_left_linear_numeric_grammar_to_regex() {
        let grammar = parse_grammar("S -> S1 | S2 | S0 | ε").unwrap();

        assert_eq!(to_regex(&grammar).unwrap(), "(1|2|0)*");
    }

    #[test]
    fn rejects_a_self_recursive_grammar_with_no_terminating_alternative() {
        let grammar = parse_grammar("S -> aS").unwrap();
        let result = to_regex(&grammar);

        assert!(matches!(
            result,
            Err(GrammarError::RegexConversionFailed(message)) if message.contains("não produz")
        ));
    }

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

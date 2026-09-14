use std::collections::{HashMap, HashSet};

use crate::symbol::{GrammarError, Symbol};

#[derive(Debug, Clone)]
pub struct Grammar {
    pub non_terminals: Vec<String>,
    pub terminals: Vec<char>,
    pub productions: HashMap<String, Vec<Vec<Symbol>>>,
    pub start: String,
}

impl Grammar {
    pub fn alternatives(&self, non_terminal: &str) -> Option<&Vec<Vec<Symbol>>> {
        self.productions.get(non_terminal)
    }
}

pub fn parse_grammar(text: &str) -> Result<Grammar, GrammarError> {
    let mut non_terminals_order: Vec<String> = Vec::new();
    let mut non_terminals_set: HashSet<String> = HashSet::new();
    let mut terminals_set: HashSet<char> = HashSet::new();
    let mut productions: HashMap<String, Vec<Vec<Symbol>>> = HashMap::new();
    let mut start: Option<String> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (lhs, rhs) = split_production_line(line)?;

        if !is_valid_non_terminal_name(&lhs) {
            return Err(GrammarError::ParseError(format!(
                "lado esquerdo inválido: '{lhs}' (use uma única letra maiúscula)"
            )));
        }

        if start.is_none() {
            start = Some(lhs.clone());
        }
        if non_terminals_set.insert(lhs.clone()) {
            non_terminals_order.push(lhs.clone());
        }

        let mut alternatives = Vec::new();
        for alt_text in rhs.split('|') {
            let alt_text = alt_text.trim();
            let symbols = parse_alternative(alt_text)?;
            for symbol in &symbols {
                if let Symbol::Terminal(c) = symbol {
                    terminals_set.insert(*c);
                }
            }
            alternatives.push(symbols);
        }

        productions.entry(lhs).or_default().extend(alternatives);
    }

    let start = start.ok_or(GrammarError::EmptyGrammar)?;

    for alts in productions.values() {
        for alt in alts {
            for symbol in alt {
                if let Symbol::NonTerminal(name) = symbol
                    && !non_terminals_set.contains(name)
                {
                    return Err(GrammarError::UndefinedNonTerminal(name.clone()));
                }
            }
        }
    }

    let mut terminals: Vec<char> = terminals_set.into_iter().collect();
    terminals.sort_unstable();

    Ok(Grammar {
        non_terminals: non_terminals_order,
        terminals,
        productions,
        start,
    })
}

fn split_production_line(line: &str) -> Result<(String, String), GrammarError> {
    let (lhs, rhs) = if let Some(idx) = line.find("::=") {
        (&line[..idx], &line[idx + 3..])
    } else if let Some(idx) = line.find("->") {
        (&line[..idx], &line[idx + 2..])
    } else {
        return Err(GrammarError::ParseError(format!(
            "linha sem separador '->' ou '::=': '{line}'"
        )));
    };
    Ok((lhs.trim().to_string(), rhs.trim().to_string()))
}

fn is_valid_non_terminal_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => chars.next().is_none(),
        _ => false,
    }
}

fn parse_alternative(alt_text: &str) -> Result<Vec<Symbol>, GrammarError> {
    if alt_text == "&" || alt_text.is_empty() {
        return Ok(Vec::new());
    }

    let mut symbols = Vec::new();
    for c in alt_text.chars() {
        if c.is_ascii_lowercase() {
            symbols.push(Symbol::Terminal(c));
        } else if c.is_ascii_uppercase() {
            symbols.push(Symbol::NonTerminal(c.to_string()));
        } else {
            return Err(GrammarError::ParseError(format!(
                "símbolo inválido '{c}' em '{alt_text}' (use apenas a-z para terminais e A-Z para não-terminais)"
            )));
        }
    }
    Ok(symbols)
}

pub fn validate_regular(grammar: &Grammar) -> Result<(), GrammarError> {
    for non_terminal in &grammar.non_terminals {
        let alternatives = grammar.productions.get(non_terminal).into_iter().flatten();
        for alt in alternatives {
            for (index, symbol) in alt.iter().enumerate() {
                let is_last = index + 1 == alt.len();
                if matches!(symbol, Symbol::NonTerminal(_)) && !is_last {
                    return Err(GrammarError::NotRegular {
                        non_terminal: non_terminal.clone(),
                        alternative: format_alternative(alt),
                        reason: "não-terminal só pode aparecer como último símbolo (gramática regular à direita)".to_string(),
                    });
                }
            }
        }
    }
    Ok(())
}

fn format_alternative(alt: &[Symbol]) -> String {
    if alt.is_empty() {
        "&".to_string()
    } else {
        alt.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn parses_pdf_example_grammar() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        assert_eq!(grammar.start, "S");
        assert_eq!(grammar.non_terminals, vec!["S".to_string()]);
        assert_eq!(grammar.terminals, vec!['a', 'b']);
        let alts = grammar.alternatives("S").unwrap();
        assert_eq!(alts.len(), 2);
        assert_eq!(
            alts[0],
            vec![Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]
        );
        assert_eq!(alts[1], vec![Symbol::Terminal('a'), Symbol::Terminal('b')]);
    }

    #[test]
    fn accepts_pdf_notation_with_double_colon_equals() {
        let grammar = parse_grammar("S ::= aS | ab").unwrap();
        assert_eq!(grammar.start, "S");
    }

    #[test]
    fn epsilon_alternative_parses_as_empty_symbol_list() {
        let grammar = parse_grammar("S -> aS | &").unwrap();
        let alts = grammar.alternatives("S").unwrap();
        assert!(alts.iter().any(|alt| alt.is_empty()));
    }

    #[test]
    fn merges_multiple_lines_for_the_same_non_terminal() {
        let grammar = parse_grammar("S -> aA\nS -> b\nA -> a").unwrap();
        assert_eq!(grammar.alternatives("S").unwrap().len(), 2);
    }

    #[test]
    fn rejects_reference_to_undeclared_non_terminal() {
        let err = parse_grammar("S -> aB").unwrap_err();
        assert_eq!(err, GrammarError::UndefinedNonTerminal("B".to_string()));
    }

    #[test]
    fn rejects_line_without_separator() {
        let err = parse_grammar("S aS").unwrap_err();
        assert!(matches!(err, GrammarError::ParseError(_)));
    }

    #[test]
    fn rejects_empty_input() {
        let err = parse_grammar("").unwrap_err();
        assert_eq!(err, GrammarError::EmptyGrammar);
    }

    #[test]
    fn rejects_multi_character_non_terminal_names() {
        let err = parse_grammar("AB -> a").unwrap_err();
        assert!(matches!(err, GrammarError::ParseError(_)));
    }

    #[test]
    fn ignores_blank_lines_and_comments() {
        let grammar = parse_grammar("# gramática de teste\nS -> a\n\n").unwrap();
        assert_eq!(grammar.start, "S");
    }

    #[test]
    fn forward_references_to_non_terminals_declared_later_are_allowed() {
        let grammar = parse_grammar("S -> aA\nA -> b").unwrap();
        assert!(grammar.alternatives("S").is_some());
    }

    #[test]
    fn accepts_right_linear_pdf_example() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        assert!(validate_regular(&grammar).is_ok());
    }

    #[test]
    fn accepts_multi_non_terminal_right_linear_grammar() {
        let grammar = parse_grammar("S -> aA | bB\nA -> bA | aC\nB -> aB | bC\nC -> a").unwrap();
        assert!(validate_regular(&grammar).is_ok());
    }

    #[test]
    fn rejects_non_terminal_in_the_middle_of_an_alternative() {
        let grammar = parse_grammar("S -> aAb\nA -> c").unwrap();
        let err = validate_regular(&grammar).unwrap_err();
        assert!(matches!(err, GrammarError::NotRegular { .. }));
    }

    #[test]
    fn rejects_more_than_one_non_terminal_in_an_alternative() {
        let grammar = parse_grammar("S -> AB\nA -> a\nB -> b").unwrap();
        let err = validate_regular(&grammar).unwrap_err();
        assert!(matches!(err, GrammarError::NotRegular { .. }));
    }
}

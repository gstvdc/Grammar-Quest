use std::collections::{HashMap, HashSet};

use rand::Rng;

use crate::symbol::{GrammarError, Symbol};

#[derive(Debug, Clone)]
pub struct Grammar {
    pub non_terminals: Vec<String>,
    pub terminals: Vec<char>,
    pub productions: HashMap<String, Vec<Vec<Symbol>>>,
    pub start: String,
}

/// Human-readable projection of `G={N,T,P,S}` so the UI can show the four
/// components literally instead of leaving the reader to infer them from raw
/// production text.
#[derive(Debug, Clone, PartialEq)]
pub struct GrammarOverview {
    pub non_terminals: String,
    pub terminals: String,
    pub start: String,
    pub production_lines: Vec<String>,
    pub production_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomGrammarConfig {
    pub non_terminal_count: usize,
    pub alternatives_per_non_terminal: usize,
    pub min_terminals_per_production: usize,
    pub max_terminals_per_production: usize,
    pub terminal_count: usize,
}

impl Grammar {
    pub fn alternatives(&self, non_terminal: &str) -> Option<&Vec<Vec<Symbol>>> {
        self.productions.get(non_terminal)
    }

    pub fn overview(&self) -> GrammarOverview {
        let production_lines: Vec<String> = self
            .non_terminals
            .iter()
            .filter_map(|nt| {
                self.alternatives(nt).map(|alts| {
                    let rhs = alts
                        .iter()
                        .map(|alt| format_alternative(alt))
                        .collect::<Vec<_>>()
                        .join(" | ");
                    format!("{nt} -> {rhs}")
                })
            })
            .collect();
        let production_count = self
            .non_terminals
            .iter()
            .filter_map(|nt| self.alternatives(nt))
            .map(|alts| alts.len())
            .sum();

        GrammarOverview {
            non_terminals: format!("{{{}}}", self.non_terminals.join(",")),
            terminals: format!(
                "{{{}}}",
                self.terminals
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            start: self.start.clone(),
            production_lines,
            production_count,
        }
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

/// Builds a productive right-linear grammar for challenge sessions. Every
/// non-terminal has a terminal-only exit and continuation alternatives, so a
/// caller can safely request short or long stack derivations.
pub fn generate_random_regular_grammar(
    config: RandomGrammarConfig,
) -> Result<Grammar, GrammarError> {
    if !(2..=26).contains(&config.non_terminal_count)
        || !(2..=8).contains(&config.alternatives_per_non_terminal)
        || !(1..=26).contains(&config.terminal_count)
        || config.min_terminals_per_production == 0
        || config.min_terminals_per_production > config.max_terminals_per_production
    {
        return Err(GrammarError::ParseError(
            "configuração inválida para gramática aleatória".to_string(),
        ));
    }

    let mut non_terminals = vec!["S".to_string()];
    non_terminals.extend(
        ('A'..='Z')
            .filter(|letter| *letter != 'S')
            .take(config.non_terminal_count - 1)
            .map(|letter| letter.to_string()),
    );
    let terminals: Vec<char> = ('a'..='z').take(config.terminal_count).collect();
    let mut rng = rand::thread_rng();
    let mut productions = HashMap::new();

    for non_terminal in &non_terminals {
        let alternatives = (0..config.alternatives_per_non_terminal)
            .map(|index| {
                let terminal_len = rng.gen_range(
                    config.min_terminals_per_production..=config.max_terminals_per_production,
                );
                let mut production = (0..terminal_len)
                    .map(|_| Symbol::Terminal(terminals[rng.gen_range(0..terminals.len())]))
                    .collect::<Vec<_>>();
                if index > 0 {
                    production.push(Symbol::NonTerminal(
                        non_terminals[rng.gen_range(0..non_terminals.len())].clone(),
                    ));
                }
                production
            })
            .collect();
        productions.insert(non_terminal.clone(), alternatives);
    }

    Ok(Grammar {
        non_terminals,
        terminals,
        productions,
        start: "S".to_string(),
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
    if alt_text == "&" || alt_text == "ε" || alt_text.is_empty() {
        return Ok(Vec::new());
    }

    let mut symbols = Vec::new();
    for c in alt_text.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            symbols.push(Symbol::Terminal(c));
        } else if c.is_ascii_uppercase() {
            symbols.push(Symbol::NonTerminal(c.to_string()));
        } else {
            return Err(GrammarError::ParseError(format!(
                "símbolo inválido '{c}' em '{alt_text}' (use a-z ou 0-9 para terminais e A-Z para não-terminais)"
            )));
        }
    }
    Ok(symbols)
}

pub fn validate_regular(grammar: &Grammar) -> Result<(), GrammarError> {
    let mut orientation = None;
    for non_terminal in &grammar.non_terminals {
        let alternatives = grammar.productions.get(non_terminal).into_iter().flatten();
        for alt in alternatives {
            let non_terminal_positions: Vec<_> = alt
                .iter()
                .enumerate()
                .filter_map(|(index, symbol)| {
                    matches!(symbol, Symbol::NonTerminal(_)).then_some(index)
                })
                .collect();
            if non_terminal_positions.len() > 1 {
                return Err(GrammarError::NotRegular {
                    non_terminal: non_terminal.clone(),
                    alternative: format_alternative(alt),
                    reason: "uma produção regular pode conter no máximo um não-terminal"
                        .to_string(),
                });
            }
            if let Some(index) = non_terminal_positions.first() {
                let current = if *index == 0 {
                    "esquerda"
                } else if *index + 1 == alt.len() {
                    "direita"
                } else {
                    return Err(GrammarError::NotRegular {
                        non_terminal: non_terminal.clone(),
                        alternative: format_alternative(alt),
                        reason: "o não-terminal deve estar no início ou no fim da produção"
                            .to_string(),
                    });
                };
                if let Some(existing) = orientation
                    && existing != current
                {
                    return Err(GrammarError::NotRegular {
                        non_terminal: non_terminal.clone(),
                        alternative: format_alternative(alt),
                        reason: "não misture produções regulares à esquerda e à direita na mesma gramática".to_string(),
                    });
                }
                orientation = Some(current);
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
    fn overview_exposes_ordered_n_t_p_s_for_the_pdf_example() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let overview = grammar.overview();
        assert_eq!(overview.non_terminals, "{S}");
        assert_eq!(overview.terminals, "{a,b}");
        assert_eq!(overview.start, "S");
        assert_eq!(overview.production_lines, vec!["S -> aS | ab".to_string()]);
        assert_eq!(overview.production_count, 2);
    }

    #[test]
    fn accepts_left_linear_grammar_with_numeric_terminals() {
        let grammar = parse_grammar("S -> S1 | S2 | S0 | ε").unwrap();

        assert_eq!(grammar.terminals, vec!['0', '1', '2']);
        assert!(validate_regular(&grammar).is_ok());
    }

    #[test]
    fn overview_groups_productions_per_non_terminal_in_declaration_order() {
        let grammar = parse_grammar("S -> aA\nS -> b\nA -> a").unwrap();
        let overview = grammar.overview();
        assert_eq!(overview.non_terminals, "{S,A}");
        assert_eq!(
            overview.production_lines,
            vec!["S -> aA | b".to_string(), "A -> a".to_string()]
        );
        assert_eq!(overview.production_count, 3);
    }

    #[test]
    fn generated_challenge_grammar_is_regular_and_has_the_requested_shape() {
        let config = RandomGrammarConfig {
            non_terminal_count: 5,
            alternatives_per_non_terminal: 4,
            min_terminals_per_production: 2,
            max_terminals_per_production: 4,
            terminal_count: 4,
        };
        let grammar = generate_random_regular_grammar(config).unwrap();

        assert_eq!(grammar.non_terminals.len(), 5);
        assert_eq!(grammar.terminals.len(), 4);
        assert_eq!(grammar.overview().production_count, 20);
        assert!(validate_regular(&grammar).is_ok());
    }

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

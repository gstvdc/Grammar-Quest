use std::collections::HashSet;

use regex::Regex;

use crate::{Grammar, GrammarError, to_regex};

/// Creates distinct strings outside the language represented by `grammar`.
/// Every candidate is checked against the anchored regex before it is returned.
pub fn generate_distractors(
    grammar: &Grammar,
    sentence: &str,
    count: usize,
) -> Result<Vec<String>, GrammarError> {
    if count == 0 {
        return Ok(Vec::new());
    }

    let pattern = to_regex(grammar)?;
    let regex = Regex::new(&format!("^(?:{pattern})$"))
        .map_err(|error| GrammarError::RegexConversionFailed(error.to_string()))?;
    if !regex.is_match(sentence) {
        return Err(GrammarError::RegexConversionFailed(
            "a sentença de referência não pertence à linguagem da gramática".to_string(),
        ));
    }

    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    let terminal_set: HashSet<char> = grammar.terminals.iter().copied().collect();

    for (index, _) in sentence.char_indices() {
        let mut candidate = sentence.to_string();
        candidate.remove(index);
        candidates.push(candidate);
    }

    let markers = ['?', '#', '!', '0', 'x', 'y', 'z'];
    for marker in markers
        .into_iter()
        .chain('a'..='z')
        .chain('\u{e000}'..='\u{f8ff}')
    {
        if terminal_set.contains(&marker) {
            continue;
        }
        candidates.push(format!("{sentence}{marker}"));
        candidates.push(format!("{marker}{sentence}"));
        for (index, _) in sentence.char_indices() {
            let mut candidate = sentence.to_string();
            candidate.replace_range(index..index + 1, &marker.to_string());
            candidates.push(candidate);
        }
    }

    let mut distractors = Vec::with_capacity(count);
    for candidate in candidates {
        if candidate != sentence && seen.insert(candidate.clone()) && !regex.is_match(&candidate) {
            distractors.push(candidate);
            if distractors.len() == count {
                return Ok(distractors);
            }
        }
    }

    Err(GrammarError::RegexConversionFailed(format!(
        "não foi possível gerar {count} distratores fora da linguagem"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_grammar;
    use regex::Regex;

    #[test]
    fn distractors_are_distinct_and_rejected_by_the_grammar_regex() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let pattern = to_regex(&grammar).unwrap();
        let options = generate_distractors(&grammar, "aab", 3).unwrap();
        let regex = Regex::new(&format!("^(?:{pattern})$")).unwrap();

        assert_eq!(options.len(), 3);
        assert_eq!(
            options
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            3
        );
        assert!(options.iter().all(|option| !regex.is_match(option)));
    }

    #[test]
    fn empty_sentence_can_receive_nonmatching_distractors() {
        let grammar = parse_grammar("S -> & | a").unwrap();
        let options = generate_distractors(&grammar, "", 2).unwrap();
        let regex = Regex::new(&format!("^(?:{})$", to_regex(&grammar).unwrap())).unwrap();

        assert_eq!(options.len(), 2);
        assert!(options.iter().all(|option| !regex.is_match(option)));
    }

    #[test]
    fn every_built_in_grammar_can_supply_verified_distractors() {
        for (_, grammar) in crate::examples() {
            let sentence = crate::derive_random(&grammar).unwrap().sentence;
            let pattern = to_regex(&grammar).unwrap();
            let regex = Regex::new(&format!("^(?:{pattern})$")).unwrap();
            let options = generate_distractors(&grammar, &sentence, 2).unwrap();

            assert_eq!(options.len(), 2);
            assert!(options.iter().all(|option| !regex.is_match(option)));
        }
    }
}

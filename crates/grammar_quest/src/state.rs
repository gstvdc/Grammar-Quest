#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_the_professor_example_selected() {
        let state = AppState::new();
        assert_eq!(state.grammar_text, "S -> aS | ab");
        assert_eq!(state.selected_example, Some(0));
        assert!(state.result.is_none());
    }

    #[test]
    fn generation_exposes_sentence_and_pdf_regex() {
        let mut state = AppState::new();
        state.generate();
        let result = state.result.expect("valid default grammar generates");
        assert_eq!(result.regex, "a*ab");
        assert!(!result.sentence.is_empty());
        assert!(state.error_message.is_none());
    }
}
use grammar_engine::{
    DerivationStep, EXAMPLE_SOURCES, derive_random, parse_grammar, to_regex, validate_regular,
};

pub struct PanelResult {
    pub sentence: String,
    pub regex: String,
    pub steps: Vec<DerivationStep>,
}

pub struct AppState {
    pub grammar_text: String,
    pub selected_example: Option<usize>,
    pub result: Option<PanelResult>,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            grammar_text: EXAMPLE_SOURCES[0].source.to_owned(),
            selected_example: Some(0),
            result: None,
            error_message: None,
        }
    }

    pub fn set_grammar_text(&mut self, text: String) {
        self.grammar_text = text;
        self.selected_example = None;
        self.result = None;
        self.error_message = None;
    }

    pub fn generate(&mut self) {
        self.result = None;
        self.error_message = None;
        let outcome = (|| {
            let grammar = parse_grammar(&self.grammar_text)?;
            validate_regular(&grammar)?;
            let derivation = derive_random(&grammar)?;
            let regex = to_regex(&grammar)?;
            Ok::<_, grammar_engine::GrammarError>(PanelResult {
                sentence: derivation.sentence,
                regex,
                steps: derivation.steps,
            })
        })();
        match outcome {
            Ok(result) => self.result = Some(result),
            Err(error) => self.error_message = Some(error.to_string()),
        }
    }
}

use grammar_engine::{
    DerivationState, DerivationStep, EXAMPLE_SOURCES, Grammar, derive_random, parse_grammar,
    to_regex, validate_regular,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenMode {
    Laboratory,
    Playing,
    Won,
}

#[derive(Debug, Clone)]
pub struct PanelResult {
    pub sentence: String,
    pub regex: String,
    pub steps: Vec<DerivationStep>,
}

pub struct AppState {
    pub mode: ScreenMode,
    pub grammar_text: String,
    pub selected_example: Option<usize>,
    pub result: Option<PanelResult>,
    pub error_message: Option<String>,
    pub current_grammar: Option<Grammar>,
    pub derivation_state: Option<DerivationState>,
    pub show_side_panel: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: ScreenMode::Laboratory,
            grammar_text: EXAMPLE_SOURCES[0].source.to_owned(),
            selected_example: Some(0),
            result: None,
            error_message: None,
            current_grammar: None,
            derivation_state: None,
            show_side_panel: true,
        }
    }

    pub fn set_grammar_text(&mut self, text: String) {
        self.grammar_text = text;
        self.selected_example = None;
        self.result = None;
        self.error_message = None;
    }

    pub fn select_example(&mut self, index: usize) {
        if let Some(example) = EXAMPLE_SOURCES.get(index) {
            self.grammar_text = example.source.to_owned();
            self.selected_example = Some(index);
            self.result = None;
            self.error_message = None;
        }
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

    pub fn start_maze(&mut self) -> Result<(), String> {
        self.error_message = None;
        let outcome = (|| -> Result<(Grammar, DerivationState), grammar_engine::GrammarError> {
            let grammar = parse_grammar(&self.grammar_text)?;
            validate_regular(&grammar)?;
            let derivation_state = DerivationState::new(&grammar);
            Ok((grammar, derivation_state))
        })();

        match outcome {
            Ok((grammar, derivation_state)) => {
                self.current_grammar = Some(grammar);
                self.derivation_state = Some(derivation_state);
                self.mode = ScreenMode::Playing;
                Ok(())
            }
            Err(err) => {
                let msg = err.to_string();
                self.error_message = Some(msg.clone());
                Err(msg)
            }
        }
    }

    pub fn apply_maze_choice(&mut self, choice: usize) -> Result<bool, String> {
        let grammar = self
            .current_grammar
            .as_ref()
            .ok_or_else(|| "Nenhuma gramática ativa no labirinto".to_string())?;
        let state = self
            .derivation_state
            .as_mut()
            .ok_or_else(|| "Nenhum estado de derivação ativo".to_string())?;

        state
            .apply_choice(grammar, choice)
            .map_err(|e| e.to_string())?;

        let is_complete = state.is_complete();
        if is_complete {
            let regex = to_regex(grammar).unwrap_or_default();
            self.result = Some(PanelResult {
                sentence: state.output.clone(),
                regex,
                steps: state.steps.clone(),
            });
        }
        Ok(is_complete)
    }

    pub fn complete_maze(&mut self) {
        if let (Some(grammar), Some(state)) = (&self.current_grammar, &self.derivation_state) {
            let regex = to_regex(grammar).unwrap_or_default();
            self.result = Some(PanelResult {
                sentence: state.output.clone(),
                regex,
                steps: state.steps.clone(),
            });
            self.mode = ScreenMode::Won;
        }
    }

    pub fn reset_maze(&mut self) {
        if let Some(grammar) = &self.current_grammar {
            self.derivation_state = Some(DerivationState::new(grammar));
            self.mode = ScreenMode::Playing;
        }
    }

    pub fn back_to_lab(&mut self) {
        self.mode = ScreenMode::Laboratory;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_the_professor_example_selected() {
        let state = AppState::new();
        assert_eq!(state.grammar_text, "S -> aS | ab");
        assert_eq!(state.selected_example, Some(0));
        assert_eq!(state.mode, ScreenMode::Laboratory);
    }

    #[test]
    fn generation_exposes_sentence_and_pdf_regex() {
        let mut state = AppState::new();
        state.generate();
        let result = state.result.expect("valid default grammar generates");
        assert_eq!(result.regex, "a*ab");
        assert!(!result.sentence.is_empty());
    }

    #[test]
    fn start_maze_transitions_to_playing() {
        let mut state = AppState::new();
        assert!(state.start_maze().is_ok());
        assert_eq!(state.mode, ScreenMode::Playing);
        assert!(state.derivation_state.is_some());
    }

    #[test]
    fn start_maze_fails_on_invalid_grammar() {
        let mut state = AppState::new();
        state.set_grammar_text("S -> aB".to_string());
        assert!(state.start_maze().is_err());
        assert_eq!(state.mode, ScreenMode::Laboratory);
        assert!(state.error_message.is_some());
    }

    #[test]
    fn maze_choices_reach_completion() {
        let mut state = AppState::new();
        state.start_maze().unwrap();
        // Choice 0 is S -> aS, choice 1 is S -> ab
        let done1 = state.apply_maze_choice(0).unwrap();
        assert!(!done1);
        let done2 = state.apply_maze_choice(1).unwrap();
        assert!(done2);
        state.complete_maze();
        assert_eq!(state.mode, ScreenMode::Won);
        let result = state.result.expect("result recorded on completion");
        assert_eq!(result.sentence, "aab");
    }
}

use grammar_engine::{
    DerivationEvent, DerivationState, DerivationStep, EXAMPLE_SOURCES, Grammar, GrammarOverview,
    RandomGrammarConfig, RegexTrace, derive_random, derive_random_in_step_range,
    generate_random_regular_grammar, parse_grammar, to_regex_trace, validate_regular,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenMode {
    Laboratory,
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
    Impossible,
}

impl Difficulty {
    pub const ALL: [Self; 5] = [
        Self::Easy,
        Self::Medium,
        Self::Hard,
        Self::Extreme,
        Self::Impossible,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "Fácil",
            Self::Medium => "Médio",
            Self::Hard => "Difícil",
            Self::Extreme => "Extremamente difícil",
            Self::Impossible => "Impossível",
        }
    }

    pub fn step_range(self) -> (usize, usize) {
        match self {
            Self::Easy => (1, 3),
            Self::Medium => (4, 6),
            Self::Hard => (7, 10),
            Self::Extreme => (11, 15),
            Self::Impossible => (16, 20),
        }
    }

    pub fn grammar_config(self) -> RandomGrammarConfig {
        match self {
            Self::Easy => RandomGrammarConfig {
                non_terminal_count: 3,
                alternatives_per_non_terminal: 2,
                min_terminals_per_production: 1,
                max_terminals_per_production: 2,
                terminal_count: 2,
            },
            Self::Medium => RandomGrammarConfig {
                non_terminal_count: 4,
                alternatives_per_non_terminal: 3,
                min_terminals_per_production: 1,
                max_terminals_per_production: 3,
                terminal_count: 3,
            },
            Self::Hard => RandomGrammarConfig {
                non_terminal_count: 5,
                alternatives_per_non_terminal: 4,
                min_terminals_per_production: 2,
                max_terminals_per_production: 4,
                terminal_count: 4,
            },
            Self::Extreme => RandomGrammarConfig {
                non_terminal_count: 7,
                alternatives_per_non_terminal: 4,
                min_terminals_per_production: 2,
                max_terminals_per_production: 5,
                terminal_count: 5,
            },
            Self::Impossible => RandomGrammarConfig {
                non_terminal_count: 9,
                alternatives_per_non_terminal: 5,
                min_terminals_per_production: 3,
                max_terminals_per_production: 6,
                terminal_count: 6,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MazeChoiceOutcome {
    Advanced,
    Restarted,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameFlow {
    Free,
    SecretChallenge,
}

/// Which of the two maze flows the editor's mode toggle currently selects.
/// Purely a UI concern — `GameFlow` is what actually drives gameplay rules
/// once a maze starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayMode {
    #[default]
    Free,
    Enigma,
}

#[derive(Debug, Clone)]
pub struct PanelResult {
    pub sentence: String,
    pub regex: String,
    pub regex_trace: RegexTrace,
    pub steps: Vec<DerivationStep>,
}

#[derive(Debug, Clone)]
struct SecretRoute {
    choices: Vec<usize>,
}

#[derive(Debug, Clone)]
struct StackAnimation {
    pushed_stack: Vec<grammar_engine::Symbol>,
    elapsed: f32,
}

const STACK_ANIMATION_SECONDS: f32 = 0.30;
const SECRET_CHECKPOINT_INTERVAL: usize = 5;
/// How long a wrong-door collision suppresses further door checks, so
/// standing on the same door's rect doesn't repeat the restart penalty
/// every frame (see docs/tasks/2026-09-15-audit-adjustments.md, T4).
const WRONG_DOOR_COOLDOWN_SECONDS: f32 = 0.6;

pub struct AppState {
    pub mode: ScreenMode,
    pub grammar_text: String,
    pub selected_example: Option<usize>,
    pub grammar_preview: Option<GrammarOverview>,
    pub grammar_is_regular: Option<bool>,
    pub selected_difficulty: Difficulty,
    pub play_mode: PlayMode,
    pub result: Option<PanelResult>,
    pub error_message: Option<String>,
    pub current_grammar: Option<Grammar>,
    pub derivation_state: Option<DerivationState>,
    pub active_flow: Option<GameFlow>,
    pub show_side_panel: bool,
    pub restart_count: u32,
    pub score: Option<u32>,
    session_regex_trace: Option<RegexTrace>,
    secret_route: Option<SecretRoute>,
    secret_progress: usize,
    stack_animation: Option<StackAnimation>,
    door_cooldown: f32,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self {
            mode: ScreenMode::Laboratory,
            grammar_text: EXAMPLE_SOURCES[0].source.to_owned(),
            selected_example: Some(0),
            grammar_preview: None,
            grammar_is_regular: None,
            selected_difficulty: Difficulty::Easy,
            play_mode: PlayMode::default(),
            result: None,
            error_message: None,
            current_grammar: None,
            derivation_state: None,
            active_flow: None,
            show_side_panel: true,
            restart_count: 0,
            score: None,
            session_regex_trace: None,
            secret_route: None,
            secret_progress: 0,
            stack_animation: None,
            door_cooldown: 0.0,
        };
        state.refresh_grammar_preview();
        state
    }

    pub fn set_grammar_text(&mut self, text: String) {
        self.grammar_text = text;
        self.selected_example = None;
        self.result = None;
        self.error_message = None;
        self.refresh_grammar_preview();
    }

    pub fn select_example(&mut self, index: usize) {
        if let Some(example) = EXAMPLE_SOURCES.get(index) {
            self.grammar_text = example.source.to_owned();
            self.selected_example = Some(index);
            self.result = None;
            self.error_message = None;
            self.refresh_grammar_preview();
        }
    }

    /// Keeps the `G={N,T,P,S}` card live as the grammar text is edited,
    /// independent of regularity/derivation validity (see
    /// docs/tasks/2026-09-15-audit-adjustments.md, T1): any parseable
    /// grammar has an `N/T/P/S` projection even if it can't be played.
    fn refresh_grammar_preview(&mut self) {
        let parsed = parse_grammar(&self.grammar_text).ok();
        self.grammar_is_regular = parsed.as_ref().map(|g| validate_regular(g).is_ok());
        self.grammar_preview = parsed.map(|g| g.overview());
    }

    pub fn generate(&mut self) {
        self.result = None;
        self.error_message = None;
        let outcome = (|| {
            let grammar = parse_grammar(&self.grammar_text)?;
            validate_regular(&grammar)?;
            let derivation = derive_random(&grammar)?;
            let regex_trace = to_regex_trace(&grammar)?;
            Ok::<_, grammar_engine::GrammarError>(PanelResult {
                sentence: derivation.sentence,
                regex: regex_trace.final_expression.clone(),
                regex_trace,
                steps: derivation.steps,
            })
        })();
        match outcome {
            Ok(result) => self.result = Some(result),
            Err(error) => self.error_message = Some(error.to_string()),
        }
    }

    pub fn start_free_maze(&mut self) -> Result<(), String> {
        self.error_message = None;
        let outcome = (|| -> Result<(Grammar, RegexTrace), grammar_engine::GrammarError> {
            let grammar = parse_grammar(&self.grammar_text)?;
            validate_regular(&grammar)?;
            // A regular-but-unproductive grammar (e.g. "S -> aB\nB -> aB") is
            // still valid per validate_regular, but has no derivable sentence.
            // Reuse derive_random's own failure signal instead of building a
            // dedicated productive-non-terminals analysis (out of scope, see
            // docs/tasks/2026-09-15-audit-adjustments.md, T2/T6).
            derive_random(&grammar)?;
            let regex_trace = to_regex_trace(&grammar)?;
            Ok((grammar, regex_trace))
        })();

        match outcome {
            Ok((grammar, regex_trace)) => {
                self.begin_maze(grammar, GameFlow::Free, None, regex_trace);
                Ok(())
            }
            Err(err) => {
                let msg = err.to_string();
                self.error_message = Some(msg.clone());
                Err(msg)
            }
        }
    }

    pub fn start_difficulty_maze(&mut self) -> Result<(), String> {
        self.error_message = None;
        let outcome =
            (|| -> Result<(Grammar, SecretRoute, RegexTrace), grammar_engine::GrammarError> {
                let grammar =
                    generate_random_regular_grammar(self.selected_difficulty.grammar_config())?;
                validate_regular(&grammar)?;
                let (min_steps, max_steps) = self.selected_difficulty.step_range();
                let target = derive_random_in_step_range(&grammar, min_steps, max_steps)?;
                // `derive_random_in_step_range` already performed an exhaustive
                // reachability search to find this route, which is a stronger
                // productivity proof than a single `derive_random` probe would be
                // (a uniform-random walk here could hit an unrelated unproductive
                // branch and falsely reject an otherwise-playable grammar) — no
                // extra productivity check needed for this flow.
                let regex_trace = to_regex_trace(&grammar)?;
                let choices = target
                    .events
                    .iter()
                    .filter_map(|event| match event {
                        DerivationEvent::ProductionChosen {
                            alternative_index, ..
                        } => Some(*alternative_index),
                        _ => None,
                    })
                    .collect();
                Ok((grammar, SecretRoute { choices }, regex_trace))
            })();

        match outcome {
            Ok((grammar, secret_route, regex_trace)) => {
                self.begin_maze(
                    grammar,
                    GameFlow::SecretChallenge,
                    Some(secret_route),
                    regex_trace,
                );
                Ok(())
            }
            Err(err) => {
                let msg = err.to_string();
                self.error_message = Some(msg.clone());
                Err(msg)
            }
        }
    }

    /// Commits a validated maze session to `AppState`. Callers must have
    /// already proven the grammar is productive and its regex trace succeeds
    /// (see `start_free_maze`/`start_difficulty_maze`) — no mutation happens
    /// here until that validation is done, so a failure never leaves the
    /// player mid-transition (T2: atomic Laboratory -> Playing commit).
    fn begin_maze(
        &mut self,
        grammar: Grammar,
        flow: GameFlow,
        secret_route: Option<SecretRoute>,
        regex_trace: RegexTrace,
    ) {
        self.derivation_state = Some(DerivationState::new(&grammar));
        self.session_regex_trace = Some(regex_trace);
        self.current_grammar = Some(grammar);
        self.mode = ScreenMode::Playing;
        self.active_flow = Some(flow);
        self.result = None;
        self.score = None;
        self.restart_count = 0;
        self.secret_progress = 0;
        self.secret_route = secret_route;
        self.stack_animation = None;
    }

    pub fn apply_maze_choice(&mut self, choice: usize) -> Result<MazeChoiceOutcome, String> {
        if self.active_flow == Some(GameFlow::SecretChallenge)
            && self.correct_secret_choice() != Some(choice)
        {
            // `restart_secret_route` sets `door_cooldown` itself.
            self.restart_secret_route()?;
            return Ok(MazeChoiceOutcome::Restarted);
        }
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
        let pushed_stack = state
            .steps
            .last()
            .map(|step| step.stack_after.clone())
            .unwrap_or_default();
        self.stack_animation = Some(StackAnimation {
            pushed_stack,
            elapsed: 0.0,
        });
        if self.active_flow == Some(GameFlow::SecretChallenge) {
            self.secret_progress += 1;
        }
        if is_complete {
            let regex_trace = to_regex_trace(grammar).map_err(|e| e.to_string())?;
            self.result = Some(PanelResult {
                sentence: state.output.clone(),
                regex: regex_trace.final_expression.clone(),
                regex_trace,
                steps: state.steps.clone(),
            });
            self.score = Some(
                1000_u32
                    .saturating_sub(state.steps.len() as u32 * 25 + self.restart_count * 150)
                    .max(100),
            );
            self.mode = ScreenMode::Won;
            return Ok(MazeChoiceOutcome::Completed);
        }
        Ok(MazeChoiceOutcome::Advanced)
    }

    pub fn advance_animations(&mut self, dt: f32) {
        if let Some(animation) = &mut self.stack_animation {
            animation.elapsed = (animation.elapsed + dt).min(STACK_ANIMATION_SECONDS);
            if animation.elapsed >= STACK_ANIMATION_SECONDS {
                self.stack_animation = None;
            }
        }
        self.door_cooldown = (self.door_cooldown - dt).max(0.0);
    }

    /// Whether a wrong-door collision is still being suppressed (see
    /// `WRONG_DOOR_COOLDOWN_SECONDS`) — standing on the door's rect must not
    /// re-trigger the restart every frame.
    pub fn door_cooldown_active(&self) -> bool {
        self.door_cooldown > 0.0
    }

    pub fn visual_stack_snapshot(&self) -> Vec<grammar_engine::Symbol> {
        if let Some(animation) = &self.stack_animation
            && animation.elapsed < STACK_ANIMATION_SECONDS
        {
            return animation.pushed_stack.clone();
        }
        self.derivation_state
            .as_ref()
            .map_or_else(Vec::new, |state| state.stack.snapshot_top_first())
    }

    pub fn stack_animation_progress(&self) -> Option<f32> {
        self.stack_animation
            .as_ref()
            .map(|animation| (animation.elapsed / STACK_ANIMATION_SECONDS).clamp(0.0, 1.0))
    }

    pub fn live_panel_result(&self) -> Option<PanelResult> {
        let state = self.derivation_state.as_ref()?;
        let regex_trace = self.session_regex_trace.clone()?;
        Some(PanelResult {
            sentence: state.output.clone(),
            regex: regex_trace.final_expression.clone(),
            regex_trace,
            steps: state.steps.clone(),
        })
    }

    pub fn correct_secret_choice(&self) -> Option<usize> {
        self.secret_route
            .as_ref()
            .and_then(|route| route.choices.get(self.secret_progress))
            .copied()
    }

    pub fn secret_progress(&self) -> usize {
        self.secret_progress
    }

    pub fn secret_total_steps(&self) -> usize {
        self.secret_route
            .as_ref()
            .map_or(0, |route| route.choices.len())
    }

    /// The furthest checkpoint already banked — the door a wrong choice
    /// rewinds to instead of the very start of the route.
    pub fn secret_last_checkpoint(&self) -> usize {
        (self.secret_progress / SECRET_CHECKPOINT_INTERVAL) * SECRET_CHECKPOINT_INTERVAL
    }

    /// Rewinds to `secret_last_checkpoint` instead of door 0: rebuilds the
    /// derivation from scratch, then replays the route's own already-proven
    /// choices up to the checkpoint (same productions `start_difficulty_maze`
    /// validated when the route was generated, so replay cannot fail).
    fn restart_secret_route(&mut self) -> Result<(), String> {
        let grammar = self
            .current_grammar
            .as_ref()
            .ok_or_else(|| "Nenhuma gramática ativa no labirinto".to_string())?;
        let checkpoint = self.secret_last_checkpoint();
        let mut state = DerivationState::new(grammar);
        if let Some(route) = &self.secret_route {
            for &choice in &route.choices[..checkpoint] {
                state
                    .apply_choice(grammar, choice)
                    .map_err(|e| e.to_string())?;
            }
        }
        self.derivation_state = Some(state);
        self.secret_progress = checkpoint;
        self.restart_count += 1;
        self.stack_animation = None;
        self.door_cooldown = WRONG_DOOR_COOLDOWN_SECONDS;
        Ok(())
    }

    pub fn reset_maze(&mut self) {
        match self.active_flow {
            Some(GameFlow::Free) => {
                let _ = self.start_free_maze();
            }
            Some(GameFlow::SecretChallenge) => {
                let _ = self.start_difficulty_maze();
            }
            None => {}
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
    fn starts_with_a_grammar_preview_for_the_default_example() {
        let state = AppState::new();
        let overview = state.grammar_preview.expect("default example parses");
        assert_eq!(overview.non_terminals, "{S}");
        assert_eq!(overview.terminals, "{a,b}");
        assert_eq!(overview.start, "S");
        assert_eq!(overview.production_count, 2);
        assert_eq!(state.grammar_is_regular, Some(true));
    }

    #[test]
    fn a_parseable_but_non_regular_grammar_is_flagged_in_the_preview() {
        let mut state = AppState::new();
        state.set_grammar_text("S -> AB\nA -> a\nB -> b".to_string());
        assert!(state.grammar_preview.is_some());
        assert_eq!(state.grammar_is_regular, Some(false));
    }

    #[test]
    fn invalid_grammar_text_clears_the_preview() {
        let mut state = AppState::new();
        state.set_grammar_text("S aS".to_string());
        assert!(state.grammar_preview.is_none());
    }

    #[test]
    fn valid_edits_refresh_the_preview_live() {
        let mut state = AppState::new();
        state.set_grammar_text("S -> aA\nA -> b".to_string());
        let overview = state.grammar_preview.expect("edited grammar parses");
        assert_eq!(overview.non_terminals, "{S,A}");
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
    fn generation_exposes_the_equation_and_elimination_trace() {
        let mut state = AppState::new();
        state.generate();
        let result = state.result.expect("valid default grammar generates");
        assert_eq!(
            result.regex_trace.initial_equations,
            vec![("S".to_string(), "S=aS+ab".to_string())]
        );
        assert_eq!(
            result.regex_trace.eliminations[0].resolved_equation,
            "S=a*ab"
        );
        assert_eq!(result.regex_trace.final_expression, "a*ab");
    }

    #[test]
    fn start_free_maze_transitions_to_playing() {
        let mut state = AppState::new();
        assert!(state.start_free_maze().is_ok());
        assert_eq!(state.mode, ScreenMode::Playing);
        assert!(state.derivation_state.is_some());
        assert_eq!(state.active_flow, Some(GameFlow::Free));
        assert_eq!(state.secret_total_steps(), 0);
    }

    #[test]
    fn difficulty_maze_generates_a_secret_route_from_a_new_grammar() {
        let mut state = AppState::new();
        state.selected_difficulty = Difficulty::Medium;
        state.start_difficulty_maze().unwrap();

        assert_eq!(state.active_flow, Some(GameFlow::SecretChallenge));
        assert!((4..=6).contains(&state.secret_total_steps()));
        assert_eq!(
            state.current_grammar.as_ref().unwrap().non_terminals.len(),
            4
        );
    }

    #[test]
    fn free_maze_finishes_when_the_stack_becomes_empty() {
        let mut state = AppState::new();
        state.start_free_maze().unwrap();

        assert_eq!(
            state.apply_maze_choice(0).unwrap(),
            MazeChoiceOutcome::Advanced
        );
        assert_eq!(
            state.apply_maze_choice(1).unwrap(),
            MazeChoiceOutcome::Completed
        );
        assert_eq!(state.mode, ScreenMode::Won);
        assert_eq!(state.result.as_ref().unwrap().sentence, "aab");
    }

    #[test]
    fn start_free_maze_fails_on_invalid_grammar() {
        let mut state = AppState::new();
        state.set_grammar_text("S -> aB".to_string());
        assert!(state.start_free_maze().is_err());
        assert_eq!(state.mode, ScreenMode::Laboratory);
        assert!(state.error_message.is_some());
    }

    #[test]
    fn start_free_maze_fails_on_an_unproductive_grammar() {
        // Regular per validate_regular (right-linear), but B never reaches a
        // terminal-only alternative: no sentence is derivable. Before the T2
        // fix this silently reached ScreenMode::Playing with a discarded
        // regex-trace failure (see docs/tasks/2026-09-15-audit-adjustments.md).
        let mut state = AppState::new();
        state.set_grammar_text("S -> aB\nB -> aB".to_string());
        assert!(state.start_free_maze().is_err());
        assert_eq!(state.mode, ScreenMode::Laboratory);
        assert!(state.error_message.is_some());
        assert!(state.current_grammar.is_none());
        assert!(state.derivation_state.is_none());
    }

    #[test]
    fn a_wrong_secret_door_restarts_the_same_route() {
        let mut state = AppState::new();
        state.selected_difficulty = Difficulty::Medium;
        state.start_difficulty_maze().unwrap();
        let correct = state.correct_secret_choice().unwrap();
        let wrong = if correct == 0 { 1 } else { 0 };

        assert_eq!(
            state.apply_maze_choice(wrong).unwrap(),
            MazeChoiceOutcome::Restarted
        );
        assert_eq!(state.restart_count, 1);
        assert_eq!(state.secret_progress(), 0);
        assert!(state.derivation_state.as_ref().unwrap().steps.is_empty());
    }

    #[test]
    fn a_wrong_door_past_the_first_checkpoint_rewinds_to_it_instead_of_the_start() {
        let mut state = AppState::new();
        state.selected_difficulty = Difficulty::Hard; // 7-10 steps, room for 6 correct ones.
        state.start_difficulty_maze().unwrap();

        for _ in 0..6 {
            let correct = state.correct_secret_choice().unwrap();
            state.apply_maze_choice(correct).unwrap();
        }
        assert_eq!(state.secret_progress(), 6);

        let correct = state.correct_secret_choice().unwrap();
        let wrong = if correct == 0 { 1 } else { 0 };
        assert_eq!(
            state.apply_maze_choice(wrong).unwrap(),
            MazeChoiceOutcome::Restarted
        );

        // Checkpoint every 5 doors: 6 correct doors banked checkpoint 5, not 0.
        assert_eq!(state.secret_progress(), 5);
        assert_eq!(state.derivation_state.as_ref().unwrap().steps.len(), 5);
    }

    #[test]
    fn completing_the_secret_route_reveals_the_sentence_and_wins_directly() {
        let mut state = AppState::new();
        state.selected_difficulty = Difficulty::Medium;
        state.start_difficulty_maze().unwrap();

        while let Some(choice) = state.correct_secret_choice() {
            state.apply_maze_choice(choice).unwrap();
        }

        assert_eq!(state.mode, ScreenMode::Won);
        assert_eq!(state.secret_progress(), state.secret_total_steps());
        assert!(
            state
                .result
                .as_ref()
                .is_some_and(|result| !result.sentence.is_empty())
        );
        assert!(state.score.unwrap() > 0);
    }

    #[test]
    fn stack_animation_shows_the_pushed_stack_then_settles() {
        let mut state = AppState::new();
        state.start_free_maze().unwrap();
        state.apply_maze_choice(0).unwrap();

        assert_eq!(state.visual_stack_snapshot().len(), 2);
        state.advance_animations(0.31);
        assert_eq!(state.visual_stack_snapshot().len(), 1);
    }
}

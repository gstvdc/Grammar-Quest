# Grammar Quest Phase 1 App Panel Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Deliver the usable Neon Arcade application shell that accepts a regular grammar, generates a random derivation, and displays sentence, regex, stack and every derivation step.

**Architecture:** Keep all formal-language behavior in `grammar_engine`. Add a pure, unit-tested `AppState` in `grammar_quest`, then narrow egui rendering modules for theme, editor and side panel; `main.rs` only composes them over macroquad.

**Tech Stack:** Rust 2024, macroquad 0.4.16, egui 0.31, egui-macroquad 0.17.3, existing `grammar_engine`; no new dependencies.

**Spec:** `docs/superpowers/specs/2026-09-14-phase1-app-panel-design.md`

## Global Constraints

- Preserve `grammar_engine` as the single owner of parsing, validation, random derivation and regex conversion.
- No new dependencies and no production `unwrap()`/`expect()` for user-provided grammar text.
- Default text is `S -> aS | ab`; all three `EXAMPLE_SOURCES` presets remain selectable.
- UI errors display the Portuguese `GrammarError` text inline and clear stale results.
- The side panel shows each production, partial output and stack top→base after production push.
- Theme is Neon Arcade: nearly black/purple background, ciano action, magenta selection, green success; color is never the only error signal.

---

### Task 1: Testable application state

**Files:** Create `crates/grammar_quest/src/state.rs`; modify `crates/grammar_quest/src/main.rs` with `mod state;`.

**Interfaces:** Produce:

```rust
pub struct PanelResult {
    pub sentence: String,
    pub regex: String,
    pub steps: Vec<grammar_engine::DerivationStep>,
}

pub struct AppState {
    pub grammar_text: String,
    pub selected_example: Option<usize>,
    pub result: Option<PanelResult>,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self;
    pub fn select_example(&mut self, index: usize);
    pub fn set_grammar_text(&mut self, text: String);
    pub fn generate(&mut self);
}
```

- [ ] **Step 1: Write failing state tests** in `state.rs` for default professor text/preset, selecting preset index 1, clearing output after manual edit, successful PDF generation (`regex == "a*ab"`), and invalid text yielding Portuguese error with `result.is_none()`.
- [ ] **Step 2: Verify RED.** Run `cargo test -p grammar_quest`; expect unresolved `AppState`/`PanelResult` compilation errors.
- [ ] **Step 3: Implement minimally.** Use `EXAMPLE_SOURCES`, `parse_grammar`, `validate_regular`, `derive_random`, and `to_regex` in that order. `select_example` uses `EXAMPLE_SOURCES.get(index)` and only mutates for a valid index. `set_grammar_text` sets `selected_example = None`, clears `result` and `error_message`. `generate` clears prior state first, stores all successful values in `PanelResult`, or stores `err.to_string()` with no result.
- [ ] **Step 4: Verify GREEN.** Run `cargo test -p grammar_quest`; all state tests pass.
- [ ] **Step 5: Commit.** `git add crates/grammar_quest/src/state.rs crates/grammar_quest/src/main.rs && git commit -m "feat(grammar_quest): add testable grammar workspace state"`

### Task 2: Neon Arcade theme and editor

**Files:** Create `crates/grammar_quest/src/ui/mod.rs`, `ui/theme.rs`, `ui/editor.rs`; modify `main.rs` with `mod ui;`.

**Interfaces:** `pub fn apply(ctx: &egui::Context)`, `pub fn show_editor(ui: &mut egui::Ui, state: &mut AppState) -> bool`. The boolean is true only when the Generate button was clicked.

- [ ] **Step 1: Write failing compile-focused unit test** in `ui/editor.rs` that instantiates `AppState`, calls `show_editor` inside an `egui::Context::run`, and verifies the function is available.
- [ ] **Step 2: Verify RED.** Run `cargo test -p grammar_quest`; expect missing `ui` modules/functions.
- [ ] **Step 3: Implement minimally.** `theme::apply` sets dark visuals, ciano `selection.bg_fill`/primary button color, magenta focus stroke and rounded cards. `show_editor` renders heading “GRAMMAR MAZE”, a ComboBox with `EXAMPLE_SOURCES` names plus “Editor manual”, a multiline `TextEdit` with 10 rows, inline `⚠` error text, and button “Gerar sentença aleatória”. On changed text call `set_grammar_text`; on preset selection call `select_example`.
- [ ] **Step 4: Verify GREEN.** Run `cargo test -p grammar_quest` and `cargo fmt -- --check`.
- [ ] **Step 5: Commit.** `git add crates/grammar_quest/src/ui crates/grammar_quest/src/main.rs && git commit -m "feat(grammar_quest): add neon grammar editor"`

### Task 3: Result workspace and derivation side panel

**Files:** Create `crates/grammar_quest/src/ui/side_panel.rs`; modify `ui/mod.rs`.

**Interfaces:** `pub fn show_result(ui: &mut egui::Ui, result: Option<&PanelResult>)` and `pub fn show_side_panel(ctx: &egui::Context, result: Option<&PanelResult>)`.

- [ ] **Step 1: Write failing rendering tests** using `egui::Context::run`: one empty-state invocation and one `PanelResult` with `S -> aS`, `output_so_far == ""`, `stack_after == vec![Terminal('a'), NonTerminal("S")]`; both must compile and complete without panic.
- [ ] **Step 2: Verify RED.** Run `cargo test -p grammar_quest`; expect absent `show_result`/`show_side_panel`.
- [ ] **Step 3: Implement minimally.** Center result card displays “SENTENÇA GERADA”, sentence, “EXPRESSÃO REGULAR” and regex; absent result shows first-use instructions. Use `egui::SidePanel::right("derivation_trace")`, `ScrollArea::vertical`, numbered cards with `non_terminal → production`, escaped top→base stack via `Symbol::to_string`, and “Saída parcial”.
- [ ] **Step 4: Verify GREEN.** Run `cargo test -p grammar_quest`.
- [ ] **Step 5: Commit.** `git add crates/grammar_quest/src/ui && git commit -m "feat(grammar_quest): render derivation result and stack trace"`

### Task 4: Compose and manually verify the desktop app

**Files:** Modify `crates/grammar_quest/src/main.rs`; modify `ROADMAP.md` only after verification.

- [ ] **Step 1: Replace the scaffold loop.** Keep `window_conf`, construct `let mut state = AppState::new();`, draw a nearly black purple macroquad background, call `theme::apply(ctx)`, render the editor in a left `egui::SidePanel`, call `state.generate()` only for its click result, render result workspace and right trace panel, then call `egui_macroquad::draw()` once per frame. Preserve `Escape` exit.
- [ ] **Step 2: Verify integration.** Run `cargo test`, `cargo fmt -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo build`; all must pass.
- [ ] **Step 3: Manual verification.** Run `cargo run -p grammar_quest`; select all three presets, generate each, edit to invalid `S -> aB`, verify inline error/no stale result, then restore `S -> aS | ab` and verify sentence, `a*ab`, stack and every step render.
- [ ] **Step 4: Mark Fase 1 complete** in `ROADMAP.md` only when the manual check passed.
- [ ] **Step 5: Commit.** `git add crates/grammar_quest/src/main.rs ROADMAP.md && git commit -m "feat(grammar_quest): deliver Phase 1 derivation workspace"`

## Plan self-review

- Spec coverage: Tasks 1–4 cover state, text/preset input, generation, inline errors, result, regex, stack, all steps, Neon Arcade layout, automated checks and manual UI validation.
- Placeholder scan: no unfilled markers or implicit error-handling steps remain.
- Type consistency: Task 1 defines the state/result that Tasks 2–3 consume; Task 4 only composes their public functions.

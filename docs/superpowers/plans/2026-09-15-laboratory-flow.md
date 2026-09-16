# Laboratory Flow Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the three-column laboratory with mode-specific setup menus, an on-demand grammar editor, and a formal-result screen.

**Architecture:** Keep `ScreenMode::Laboratory` and add `LaboratoryStage` to `AppState`. A new `ui::laboratory` module renders setup, example selection, grammar editing, and results as a single centered surface; `app.rs` maps actions to existing grammar operations and room creation.

**Tech Stack:** Rust 2024, macroquad 0.4.16, egui 0.31, egui-macroquad 0.17.3.

**Spec:** `docs/superpowers/specs/2026-09-15-laboratory-flow-design.md`

## Global Constraints

- Do not modify `grammar_engine` or duplicate parsing, derivation, or regex logic in the game crate.
- Keep `ui::side_panel` as the shared on-demand maze trace.
- Preserve grammar input, all three examples, random derivation, stack trace, `G={N,T,P,S}`, and regex conversion.
- Add no dependencies and retain the 16-bit purple/scanline visual language.
- Finish with `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.

---

### Task 1: Model laboratory stages in application state

**Files:**
- Modify: `crates/grammar_quest/src/state.rs`
- Test: `crates/grammar_quest/src/state.rs`

**Interfaces:**
- Produces `LaboratoryStage::{Setup, ExampleSelection, GrammarEditor, FormalResult}`.
- Produces `AppState::open_laboratory_stage`, `AppState::return_to_setup`, and stage-aware `handle_escape`.

- [ ] **Step 1: Write failing state tests**

  ```rust
  #[test]
  fn entering_a_play_mode_opens_its_setup_stage() {
      let mut state = AppState::new();
      state.select_play_mode(PlayMode::Free);
      assert_eq!(state.laboratory_stage, LaboratoryStage::Setup);
  }

  #[test]
  fn escape_from_editor_returns_to_setup_without_losing_grammar_text() {
      let mut state = AppState::new();
      state.select_play_mode(PlayMode::Free);
      state.set_grammar_text("S -> a".to_owned());
      state.open_laboratory_stage(LaboratoryStage::GrammarEditor);
      state.handle_escape();
      assert_eq!(state.laboratory_stage, LaboratoryStage::Setup);
      assert_eq!(state.grammar_text, "S -> a");
  }
  ```

- [ ] **Step 2: Verify the tests fail**

  Run: `cargo test -p grammar_quest state::tests::entering_a_play_mode_opens_its_setup_stage -- --exact`

  Expected: `LaboratoryStage` and `laboratory_stage` are missing.

- [ ] **Step 3: Implement state transitions**

  Add the enum, field default, and methods. `select_play_mode` sets `mode = Laboratory` and `laboratory_stage = Setup`; `handle_escape` returns Editor, ExampleSelection, and FormalResult to Setup before applying the existing screen-level navigation.

- [ ] **Step 4: Run state tests**

  Run: `cargo test -p grammar_quest state::tests`

  Expected: all state tests pass.

### Task 2: Replace permanent laboratory panels with a setup menu

**Files:**
- Create: `crates/grammar_quest/src/ui/laboratory.rs`
- Modify: `crates/grammar_quest/src/ui/mod.rs`
- Modify: `crates/grammar_quest/src/app.rs`

**Interfaces:**
- Produces `LaboratoryAction::{None, OpenExamples, OpenEditor, SelectExample(usize), Generate, PlayFree, PlayEnigma, BackToSetup}`.
- Consumes `AppState` and returns intent only; `app.rs` owns `start_*_maze` and `Room::build`.

- [ ] **Step 1: Add a UI smoke test before implementation**

  ```rust
  #[test]
  fn laboratory_setup_renders_without_a_permanent_side_panel() {
      let ctx = egui::Context::default();
      let mut state = AppState::new();
      state.select_play_mode(PlayMode::Free);
      ctx.run(egui::RawInput::default(), |ctx| {
          assert_eq!(show_laboratory(ctx, &mut state), LaboratoryAction::None);
      });
  }
  ```

- [ ] **Step 2: Verify it fails**

  Run: `cargo test -p grammar_quest ui::laboratory::tests::laboratory_setup_renders_without_a_permanent_side_panel -- --exact`

  Expected: module and function are missing.

- [ ] **Step 3: Implement centered setup rendering**

  Render one `CentralPanel` with the scanline/plum style. For Free: wide framed buttons for examples, editor, generate sentence, and play; show the active grammar name/status. For Enigma: render five clickable difficulty cards from `Difficulty::ALL` and an Enigma start button; never render grammar input/examples. Render all three examples as large selection cards containing the supplied name, description, and source.

- [ ] **Step 4: Wire actions in `run_laboratory_frame`**

  Remove `SidePanel::left`, `side_panel::show_side_panel`, and the old central welcome/result composition. Map selection to `state.select_example`, stage transitions to Task 1 methods, generation to `state.generate()` then `FormalResult` on success, and play actions to the existing room/player setup code.

- [ ] **Step 5: Verify interaction contracts**

  Run: `cargo test -p grammar_quest && cargo build --workspace`

  Expected: old `ui::editor::show_editor` no longer participates in the laboratory shell; all tests pass.

### Task 3: Move formal input and results into explicit stages

**Files:**
- Modify: `crates/grammar_quest/src/ui/editor.rs`
- Modify: `crates/grammar_quest/src/ui/side_panel.rs`
- Modify: `crates/grammar_quest/src/ui/laboratory.rs`

**Interfaces:**
- `editor.rs` produces only `EditorAction::{None, SaveAndBack}` and edits `AppState` grammar fields.
- `side_panel.rs` exports reusable `show_result` and a new `show_grammar_overview` helper; it does not create a permanent preparation panel.

- [ ] **Step 1: Write a failing editor contract test**

  ```rust
  #[test]
  fn editor_can_render_for_a_free_setup() {
      let ctx = egui::Context::default();
      let mut state = AppState::new();
      state.select_play_mode(PlayMode::Free);
      ctx.run(egui::RawInput::default(), |ctx| {
          egui::CentralPanel::default().show(ctx, |ui| {
              assert_eq!(show_editor(ui, &mut state), EditorAction::None);
          });
      });
  }
  ```

- [ ] **Step 2: Implement the stage components**

  Remove the old editor header, mode controls, CTA buttons, and difficulty selector. Keep the multiline grammar field, error card, and `G={N,T,P,S}` card. In `FormalResult`, call `show_grammar_overview` and `show_result`; add `Jogar labirinto` and `Voltar` actions.

- [ ] **Step 3: Verify the mandatory academic flow**

  Run: `cargo test --workspace`

  Expected: examples still parse, generated result still has sentence and regex, and the new UI smoke test passes.

### Task 4: Update documents and run final visual gate

**Files:**
- Modify: `README.md`
- Modify: `DESIGN.md`
- Modify: `docs/superpowers/specs/2026-09-15-laboratory-flow-design.md`

- [ ] **Step 1: Update navigation documentation**

  Describe the selection flow instead of a laboratory panel, including where a user enters productions and opens the formal result.

- [ ] **Step 2: Update design records**

  Document that the menu visual language is reused in preparation screens and that the formal panel is on-demand.

- [ ] **Step 3: Run final checks**

  ```bash
  cargo fmt --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  git diff --check
  ```

  Expected: no warnings, failures, or whitespace errors.

# Grammar Quest

<p align="center">
  <img src="crates/grammar_quest/assets/brand/grammar-quest-logo.png" alt="Grammar Quest logo" width="220" />
</p>

Grammar Quest is the TD1 development assignment for the Formal Languages and
Automata course at UNESC, taught by Professor André Faria Ruaro. The project
turns regular grammar derivation into a 2D maze: each door represents a
production, and the player's choice performs the actual derivation using a
stack.

The original assignment is available at
[`context/Linguagens Formais - Aula 6 - TD01.pdf`](context/Linguagens%20Formais%20-%20Aula%206%20-%20TD01.pdf).

## Assignment requirements

- accept a grammar `G = {N, T, P, S}`;
- generate random sentences;
- derive sentences with a stack, keeping the leftmost symbol on top;
- accept only right-linear regular grammars;
- provide a graphical interface for input and derivation results;
- include three selectable example grammars;
- convert the grammar into a regular expression after derivation.

The current editor accepts productions, infers `N` and `T`, and treats the
left-hand side of the first production as `S`. The project audit recommends
making all four components explicitly visible before the academic submission.

## How it works

In the laboratory, select an example or enter productions such as:

```text
S -> aS | ab
```

The `::=` separator is also supported. Uppercase letters represent
non-terminals, lowercase letters represent terminals, and `&` represents the
empty word.

- **Generate a random sentence:** the engine chooses productions and displays
  the sentence, stack, derivation steps, and regular expression.
- **Play the maze:** each door applies an alternative for the non-terminal at
  the top of the stack. After completing the derivation, the player solves a
  language-membership challenge.

For the example from the assignment, the resulting regular expression is
`a*ab`.

## Run

Prerequisite: a stable Rust toolchain with Rust 2024 edition support.

```bash
cargo run -p grammar_quest
```

Controls:

- `WASD` or arrow keys: move;
- `Tab`: show or hide the formal derivation trace;
- `Esc`: return to the laboratory; from the laboratory, quit the application.

### macOS app shortcut

To get a double-clickable `Grammar Quest.app` (no terminal needed), with the
logo as its Dock and Finder icon:

```bash
scripts/build-macos-app.sh
```

This builds a release binary, generates `AppIcon.icns` from
`crates/grammar_quest/assets/brand/grammar-quest-logo.png`, and assembles the
bundle at `dist/Grammar Quest.app`. Re-run it after moving or deleting the
existing bundle — the script refuses to overwrite one in place.

### macOS note

In the current automation environment, the binary aborted inside
`miniquad 0.4.11` while initializing the application menu because
`NSRunningApplication.localizedName` returned null. Builds and tests work.
Before presenting the project, run it in a regular graphical session or as an
`.app` bundle and complete the visual smoke test described in the audit.

## Verify

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
```

To test only the formal grammar engine:

```bash
cargo test -p grammar_engine
```

## Architecture

```text
crates/
  grammar_engine/   parser, validation, stack, derivation, regex, distractors
  grammar_quest/    application state, maze, player, and graphical interface
    src/
      app.rs          macroquad loop, screen transitions, egui composition
      gameplay.rs      player movement, door collision, choice resolution
      effects.rs       particles, shockwaves, floating text
      render/          arena/grid/walls (world.rs) and doors/signage (portals.rs)
      state.rs, maze.rs, player.rs, ui/
context/             original assignment
docs/
  audits/            compliance and quality audits
  tasks/             implementation backlog for subagents
  superpowers/       designs and plans for implemented phases
```

`grammar_engine` has no dependency on the graphical interface.
`grammar_quest` consumes its API and must not reimplement formal-language
logic.

## Status and next steps

The project history is available in [`ROADMAP.md`](ROADMAP.md). The complete
review of compliance, code quality, UI/UX, duplication, and dead code is in
[`docs/audits/2026-09-15-project-audit.md`](docs/audits/2026-09-15-project-audit.md).
Implementation tasks prepared for delegation are available in
[`docs/tasks/2026-09-15-audit-adjustments.md`](docs/tasks/2026-09-15-audit-adjustments.md).

## Asset licenses

See
[`crates/grammar_quest/assets/ATTRIBUTION.md`](crates/grammar_quest/assets/ATTRIBUTION.md).

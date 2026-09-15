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
[`context/td01-linguagens-formais.pdf`](context/td01-linguagens-formais.pdf).

## Assignment requirements

- accept a grammar `G = {N, T, P, S}`;
- generate random sentences;
- derive sentences with a stack, keeping the leftmost symbol on top;
- accept only right-linear regular grammars;
- provide a graphical interface for input and derivation results;
- include three selectable example grammars;
- convert the grammar into a regular expression after derivation.

The editor accepts productions and shows a live `G = {N, T, P, S}` card
(non-terminals, terminals, start symbol, and every production) derived
straight from the parsed grammar — see `docs/acceptance-checklist.md` for the
exact reproduction steps and expected output.

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
  the top of the stack; picking one performs that step of the derivation with
  the character. A single mode toggle in the editor picks which maze starts:
  - **Livre:** uses the example or productions currently in the editor; the
    sentence is visible the whole time.
  - **Modo Enigma:** generates a fresh random grammar sized to the chosen
    difficulty, and its sentence stays hidden until every door is solved
    correctly. Progress checkpoints every 5 doors — a wrong door rewinds to
    the last checkpoint instead of restarting the whole route.

Sound effects (a correct-door blip, a wrong-door buzz, a victory chime, and a
UI click) play via `macroquad::audio`; see
[`crates/grammar_quest/assets/ATTRIBUTION.md`](crates/grammar_quest/assets/ATTRIBUTION.md)
for their provenance.

For the example from the assignment, the resulting regular expression is
`a*ab`.

## Run

Prerequisite: a stable Rust toolchain with 2024 edition support, verified
with `rustc 1.98.0 (88d9e12ae 2026-08-18)`. No `rust-toolchain.toml` pins a
version — any newer stable toolchain with 2024 edition support is expected to
work.

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
`.app` bundle and complete the visual smoke test described in
[`docs/acceptance-checklist.md`](docs/acceptance-checklist.md).

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
      audio.rs         embedded sound effects (macroquad::audio)
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
For a requirement-by-requirement reproduction script (what to click, what to
expect), see [`docs/acceptance-checklist.md`](docs/acceptance-checklist.md).

## Asset licenses

See
[`crates/grammar_quest/assets/ATTRIBUTION.md`](crates/grammar_quest/assets/ATTRIBUTION.md).

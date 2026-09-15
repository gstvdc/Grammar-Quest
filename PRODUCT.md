# Product

<!-- impeccable:product-schema 1 -->

## Platform

desktop

Deviation from Impeccable's web/iOS/Android/adaptive schema, confirmed with
the user: Grammar Quest is a native desktop app (a `macroquad` window with an
`egui-macroquad` overlay), not a browser or mobile app. Impeccable's
browser-driven tooling (`live`, `generate`, `detect`, the design hook) does
not apply here; design help on this project is code/screenshot review, not
automated scanning.

## Stack

Existing codebase: Rust, Cargo workspace (edition 2024), `macroquad` +
`egui-macroquad` for windowing/rendering/UI, `rand = "0.8"` pinned across
crates. Two crates: `grammar_engine` (pure formal-language logic, no UI
dependency) and `grammar_quest` (the game binary, consumes `grammar_engine`
via path dependency).

## Users

Primary user: Professor André Faria Ruaro, grading the TD1 assignment
(Linguagens Formais e Autômatos, UNESC) against the 7 explicit requirements
in `AGENTS.md`/`CLAUDE.md`. Confirmed grader-first: design and UX decisions
should optimize for clearly demonstrating correctness and completeness of
the grammar engine and derivation mechanism to an evaluator, not for a
broader player audience. Other students playing the maze is a side benefit,
not a design driver.

## Product Purpose

Grammar Quest ("Grammar Maze") turns a formal-languages homework assignment
into a playable 2D maze: each maze door represents a grammar production, and
the player's choice at that door performs a real stack-based derivation
step, reflected live in a side panel (stack state, derivation log, final
sentence, derived regular expression). Success = every one of the 7
assignment requirements is implemented and independently verifiable, and the
maze layer built on top does not compromise or duplicate that logic.

## Positioning

Unlike a plain grammar-derivation exercise (console output or a static
form), Grammar Quest makes the derivation algorithm itself the game
mechanic — walking through a maze door *is* the push/pop stack operation,
not a metaphor layered over it. A neighboring TD1 submission implementing
only the required GUI form could not truthfully claim this mechanism.

## Operating Context

- Run via `cargo run -p grammar_quest`, or as a bundled `Grammar Quest.app`
  on macOS built by `scripts/build-macos-app.sh`.
- Two screens reusing one shared UI panel: the "laboratory" (grammar
  input/example selection, random sentence generation, derivation/regex
  display — Fase 1, alone already satisfies 100% of the grading spec) and
  the maze (Fase 2/3 — doors as productions, live stack animation, typing
  log, language-membership puzzle doors, win/score screen).
- Controls: WASD/arrows to move, Tab to toggle the formal derivation trace,
  Esc to return to the laboratory (or quit from there).
- Known environment issue (not a design concern): in some automation
  environments the binary aborts during macOS app-menu init inside
  `miniquad` (`NSRunningApplication.localizedName` returns null); `cargo
  test`/clippy/build all pass. Visual verification must happen in a normal
  graphical session before submission — see
  `docs/audits/2026-09-15-project-audit.md`.

## Capabilities and Constraints

- Non-negotiable (independent of the game layer, per `AGENTS.md`): grammar
  input as `G = {N, T, P, S}`; random sentence generation; stack-based
  derivation using the exact PDF algorithm (choose production → push with
  leftmost symbol on top → pop: terminal to output, non-terminal expands
  again); regular grammars only; a graphical interface with input fields and
  derivation-result display; three selectable built-in example grammars;
  conversion to a regular expression (non-terminal elimination) after
  derivation.
- Grammar text notation: `S -> aS | ab` (also accepts `::=`); uppercase =
  non-terminal, lowercase = terminal, `&` = empty word.
- Current editor infers `N`/`T` from productions and treats the first
  production's LHS as `S` — the project audit recommends making all four
  grammar components explicitly visible before submission (an open,
  tracked gap, not yet a design decision).
- Architecture rule: `grammar_engine` has no UI dependency and must stay
  independently testable (`cargo test -p grammar_engine`); `grammar_quest`
  must not reimplement grammar logic. The Fase 1 side panel (stack /
  derivation / regex) is reused inside the maze, not duplicated.
- Scope boundary (YAGNI, per `ROADMAP.md`): context-free/context-sensitive
  grammars, multiplayer, save/load, and sound are explicitly out of scope.

## Brand Commitments

- Confirmed binding: keep the existing brand and asset work rather than
  redesigning it. This includes the Grammar Quest logo
  (`crates/grammar_quest/assets/brand/grammar-quest-logo.png`, also used as
  the macOS app icon) and the licensed player sprite under
  `crates/grammar_quest/assets/player/`, with attribution tracked in
  `crates/grammar_quest/assets/ATTRIBUTION.md`. The unused dungeon-tile
  assets flagged by the 2026-09-15 audit were removed in the same pass.
  Future design work refines around these, not through replacing them.
- Fase 3 already delivered a stated "coherent visual theme (palette,
  typography, screen transitions)" and stack push/pop tweening plus a
  typewriter-effect derivation log — treat as the incumbent visual system
  to document (via `/impeccable document`), not to redesign from scratch,
  absent a future explicit request.

## Evidence on Hand

- The original assignment PDF: `context/Linguagens Formais - Aula 6 -
  TD01.pdf`.
- Project audit (compliance, code quality, UI/UX, duplication, dead code):
  `docs/audits/2026-09-15-project-audit.md`.
- Implementation backlog derived from that audit:
  `docs/tasks/2026-09-15-audit-adjustments.md`.
- No testimonials, external users, or usage data exist or should be
  implied — this is a single-assignment academic project with one
  effective grading audience.

## Product Principles

1. The 7 graded requirements are the non-negotiable floor — no maze/game
   feature may ever obscure, duplicate, or risk breaking them.
2. The formal-language engine and the game are architecturally separate;
   design and UX work on the game layer must never leak grammar logic back
   into `grammar_quest` or bypass `grammar_engine`.
3. The derivation panel is one shared UI surface reused across both
   screens — consistency between the laboratory and the maze view is a
   correctness signal for the grader, not just polish.
4. Existing brand and licensed assets are preserved; new design work is
   refinement-in-world, not replacement, absent explicit direction
   otherwise.
5. Grader-first: clarity and demonstrability of correctness outrank
   broader-audience engagement or onboarding concerns.

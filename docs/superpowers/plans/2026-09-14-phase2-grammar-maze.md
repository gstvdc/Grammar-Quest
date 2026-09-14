# Grammar Maze Phase 2 Implementation Plan

> **For agentic workers:** Use subagent-driven-development or executing-plans task-by-task.

**Goal:** Add a playable top-down maze where entering a production door drives the real grammar stack to completion.

**Spec:** `docs/superpowers/specs/2026-09-14-phase2-grammar-maze-design.md`

## Tasks

### Task 1: Controlled derivation engine

- Add `DerivationState` and `apply_choice` to `grammar_engine`; test initial stack, terminal consumption, selected production and completion.
- Keep `derive_random` implemented through the same state transition.
- Verify `cargo test -p grammar_engine`; commit `feat(grammar_engine): add player-controlled derivation state`.

### Task 2: Asset integration

- Copy the user-provided `Swordsman_lvl1_Walk_without_shadow.png` to `crates/grammar_quest/assets/player/` and selected CC0 metal/glass tiles to `assets/tiles/`.
- Add `assets/ATTRIBUTION.md` with CraftPix source/license URL and OpenGameArt CC0 source.
- Test asset paths exist; commit `assets: add temporary maze player and CC0 tiles`.

### Task 3: Maze model and renderer

- Create pure maze-room/door geometry from current alternatives; test one door per alternative and collision rectangles.
- Add macroquad player movement, camera, animated 6×4 swordsman sprite and rendered floor/walls/doors.
- Verify build and manual movement; commit `feat(grammar_quest): render playable grammar maze`.

### Task 4: Door choices, HUD and victory

- Trigger `apply_choice` when the player enters a door; render next room, HUD and reused derivation trace.
- Consume terminals automatically and show victory when stack empties.
- Verify all presets manually plus workspace tests/fmt/clippy/build; mark Roadmap Fase 2 and commit.

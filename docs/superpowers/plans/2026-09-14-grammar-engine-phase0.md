# Grammar Engine (Phase 0) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `crates/grammar_engine`, the pure-logic core of Grammar Quest: parse a regular grammar `G = {N, T, P, S}`, validate it's regular, derive random sentences with a stack (per the professor's exact algorithm), and convert the grammar to an equivalent regular expression.

**Architecture:** Six small modules, each with one responsibility, re-exported from `lib.rs`: `symbol` (data types + errors), `grammar` (model + text parser + regularity check), `stack` (the pilha), `derivation` (random derivation using the stack), `regex_conversion` (non-terminal elimination / Arden's rule), `examples` (the three built-in grammars). No module here depends on macroquad/egui — this crate must build and test standalone.

**Tech Stack:** Rust (edition 2024), `rand = "0.8"` for random production choice, `regex = "1"` as a **dev-dependency only** (used to validate generated sentences against the derived regex in tests — not shipped in the engine's public API).

**Spec:** `docs/superpowers/specs/2026-09-14-grammar-quest-design.md`

## Global Constraints

- Edition 2024, matches `crates/grammar_engine/Cargo.toml` (already configured).
- `rand = "0.8"` is already a dependency and `regex = "1"` already a dev-dependency in `crates/grammar_engine/Cargo.toml` — do not add new dependencies in this plan.
- No `unwrap()`/`expect()` outside `#[cfg(test)]` code, **except** for the hardcoded example-grammar strings in `examples.rs`: a panic there means a programmer typo in a compile-time constant (not user input), and Task 7's own tests catch it immediately.
- Every `GrammarError` message is Portuguese, matching the rest of the project (`CLAUDE.md`, `README.md`).
- `to_regex` emits **standard regex syntax** (`|` for union, `*` for Kleene star, `(...)` for grouping) rather than the course slides' blackboard "`+`" notation. Both denote the same regular expression; standard syntax is directly usable with the `regex` crate for validation (needed later for the Fase 3 door-puzzle). For the professor's own worked example (`S ::= aS | ab`) the output is still the exact string `a*ab`, because that example never needs a union operator.
- Known, accepted limitation: a non-terminal with an epsilon *self*-production (e.g. `A -> A` alone) is not meaningfully supported. None of the three built-in example grammars use that pattern.

---

### Task 1: Core types — `Symbol` and `GrammarError`

**Files:**
- Create: `crates/grammar_engine/src/symbol.rs`
- Modify: `crates/grammar_engine/src/lib.rs` (replace placeholder doc comment with module wiring)
- Test: inline `#[cfg(test)] mod tests` in `symbol.rs`

**Interfaces:**
- Produces: `Symbol` enum (`Terminal(char)`, `NonTerminal(String)`, derives `Debug, Clone, PartialEq, Eq, Hash`, implements `Display`), `GrammarError` enum (`ParseError(String)`, `UndefinedNonTerminal(String)`, `NotRegular { non_terminal: String, alternative: String, reason: String }`, `EmptyGrammar`, `DerivationTooLong`, `RegexConversionFailed(String)`, derives `Debug, Clone, PartialEq, Eq`, implements `Display` + `std::error::Error`). Every later task consumes these two types.

- [ ] **Step 1: Write the failing tests**

Create `crates/grammar_engine/src/symbol.rs` with just the test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_displays_as_its_char() {
        assert_eq!(Symbol::Terminal('a').to_string(), "a");
    }

    #[test]
    fn non_terminal_displays_as_its_name() {
        assert_eq!(Symbol::NonTerminal("S".to_string()).to_string(), "S");
    }

    #[test]
    fn symbols_are_usable_as_hashmap_keys() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(Symbol::NonTerminal("S".to_string()), 1);
        assert_eq!(map.get(&Symbol::NonTerminal("S".to_string())), Some(&1));
    }

    #[test]
    fn not_regular_error_message_is_readable() {
        let err = GrammarError::NotRegular {
            non_terminal: "S".to_string(),
            alternative: "AB".to_string(),
            reason: "não-terminal só pode aparecer como último símbolo".to_string(),
        };
        let message = err.to_string();
        assert!(message.contains('S'));
        assert!(message.contains("AB"));
    }
}
```

Also add `mod symbol;` to `crates/grammar_engine/src/lib.rs`, replacing its current content:

```rust
mod symbol;
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `Symbol` and `GrammarError` are not defined yet.

- [ ] **Step 3: Implement the types**

Prepend this to `crates/grammar_engine/src/symbol.rs` (above the `#[cfg(test)]` block):

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(char),
    NonTerminal(String),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Terminal(c) => write!(f, "{c}"),
            Symbol::NonTerminal(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarError {
    ParseError(String),
    UndefinedNonTerminal(String),
    NotRegular {
        non_terminal: String,
        alternative: String,
        reason: String,
    },
    EmptyGrammar,
    DerivationTooLong,
    RegexConversionFailed(String),
}

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrammarError::ParseError(msg) => write!(f, "erro de sintaxe na gramática: {msg}"),
            GrammarError::UndefinedNonTerminal(name) => {
                write!(f, "não-terminal '{name}' usado mas nunca definido")
            }
            GrammarError::NotRegular {
                non_terminal,
                alternative,
                reason,
            } => write!(
                f,
                "produção '{non_terminal} -> {alternative}' não é regular: {reason}"
            ),
            GrammarError::EmptyGrammar => write!(f, "gramática vazia"),
            GrammarError::DerivationTooLong => write!(
                f,
                "derivação excedeu o limite de passos (gramática pode não ser produtiva)"
            ),
            GrammarError::RegexConversionFailed(msg) => {
                write!(f, "falha ao converter para expressão regular: {msg}")
            }
        }
    }
}

impl std::error::Error for GrammarError {}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (4 tests in `symbol::tests`).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/symbol.rs crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): add Symbol and GrammarError core types"
```

---

### Task 2: `Grammar` model and text parser

**Files:**
- Create: `crates/grammar_engine/src/grammar.rs`
- Modify: `crates/grammar_engine/src/lib.rs` (add `mod grammar;`)
- Test: inline `#[cfg(test)] mod tests` in `grammar.rs`

**Interfaces:**
- Consumes: `Symbol`, `GrammarError` (Task 1).
- Produces: `Grammar { non_terminals: Vec<String>, terminals: Vec<char>, productions: HashMap<String, Vec<Vec<Symbol>>>, start: String }` (derives `Debug, Clone`), `Grammar::alternatives(&self, non_terminal: &str) -> Option<&Vec<Vec<Symbol>>>`, `parse_grammar(text: &str) -> Result<Grammar, GrammarError>`. Consumed by every later task.

- [ ] **Step 1: Write the failing tests**

Create `crates/grammar_engine/src/grammar.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn parses_pdf_example_grammar() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        assert_eq!(grammar.start, "S");
        assert_eq!(grammar.non_terminals, vec!["S".to_string()]);
        assert_eq!(grammar.terminals, vec!['a', 'b']);
        let alts = grammar.alternatives("S").unwrap();
        assert_eq!(alts.len(), 2);
        assert_eq!(
            alts[0],
            vec![Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]
        );
        assert_eq!(alts[1], vec![Symbol::Terminal('a'), Symbol::Terminal('b')]);
    }

    #[test]
    fn accepts_pdf_notation_with_double_colon_equals() {
        let grammar = parse_grammar("S ::= aS | ab").unwrap();
        assert_eq!(grammar.start, "S");
    }

    #[test]
    fn epsilon_alternative_parses_as_empty_symbol_list() {
        let grammar = parse_grammar("S -> aS | &").unwrap();
        let alts = grammar.alternatives("S").unwrap();
        assert!(alts.iter().any(|alt| alt.is_empty()));
    }

    #[test]
    fn merges_multiple_lines_for_the_same_non_terminal() {
        let grammar = parse_grammar("S -> aA\nS -> b\nA -> a").unwrap();
        assert_eq!(grammar.alternatives("S").unwrap().len(), 2);
    }

    #[test]
    fn rejects_reference_to_undeclared_non_terminal() {
        let err = parse_grammar("S -> aB").unwrap_err();
        assert_eq!(err, GrammarError::UndefinedNonTerminal("B".to_string()));
    }

    #[test]
    fn rejects_line_without_separator() {
        let err = parse_grammar("S aS").unwrap_err();
        assert!(matches!(err, GrammarError::ParseError(_)));
    }

    #[test]
    fn rejects_empty_input() {
        let err = parse_grammar("").unwrap_err();
        assert_eq!(err, GrammarError::EmptyGrammar);
    }

    #[test]
    fn rejects_multi_character_non_terminal_names() {
        let err = parse_grammar("AB -> a").unwrap_err();
        assert!(matches!(err, GrammarError::ParseError(_)));
    }

    #[test]
    fn ignores_blank_lines_and_comments() {
        let grammar = parse_grammar("# gramática de teste\nS -> a\n\n").unwrap();
        assert_eq!(grammar.start, "S");
    }

    #[test]
    fn forward_references_to_non_terminals_declared_later_are_allowed() {
        let grammar = parse_grammar("S -> aA\nA -> b").unwrap();
        assert!(grammar.alternatives("S").is_some());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `parse_grammar` is not defined.

- [ ] **Step 3: Implement the model and parser**

Prepend this to `crates/grammar_engine/src/grammar.rs` (above `#[cfg(test)]`):

```rust
use std::collections::{HashMap, HashSet};

use crate::symbol::{GrammarError, Symbol};

#[derive(Debug, Clone)]
pub struct Grammar {
    pub non_terminals: Vec<String>,
    pub terminals: Vec<char>,
    pub productions: HashMap<String, Vec<Vec<Symbol>>>,
    pub start: String,
}

impl Grammar {
    pub fn alternatives(&self, non_terminal: &str) -> Option<&Vec<Vec<Symbol>>> {
        self.productions.get(non_terminal)
    }
}

pub fn parse_grammar(text: &str) -> Result<Grammar, GrammarError> {
    let mut non_terminals_order: Vec<String> = Vec::new();
    let mut non_terminals_set: HashSet<String> = HashSet::new();
    let mut terminals_set: HashSet<char> = HashSet::new();
    let mut productions: HashMap<String, Vec<Vec<Symbol>>> = HashMap::new();
    let mut start: Option<String> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (lhs, rhs) = split_production_line(line)?;

        if !is_valid_non_terminal_name(&lhs) {
            return Err(GrammarError::ParseError(format!(
                "lado esquerdo inválido: '{lhs}' (use uma única letra maiúscula)"
            )));
        }

        if start.is_none() {
            start = Some(lhs.clone());
        }
        if non_terminals_set.insert(lhs.clone()) {
            non_terminals_order.push(lhs.clone());
        }

        let mut alternatives = Vec::new();
        for alt_text in rhs.split('|') {
            let alt_text = alt_text.trim();
            let symbols = parse_alternative(alt_text)?;
            for symbol in &symbols {
                if let Symbol::Terminal(c) = symbol {
                    terminals_set.insert(*c);
                }
            }
            alternatives.push(symbols);
        }

        productions.entry(lhs).or_default().extend(alternatives);
    }

    let start = start.ok_or(GrammarError::EmptyGrammar)?;

    for alts in productions.values() {
        for alt in alts {
            for symbol in alt {
                if let Symbol::NonTerminal(name) = symbol {
                    if !non_terminals_set.contains(name) {
                        return Err(GrammarError::UndefinedNonTerminal(name.clone()));
                    }
                }
            }
        }
    }

    let mut terminals: Vec<char> = terminals_set.into_iter().collect();
    terminals.sort_unstable();

    Ok(Grammar {
        non_terminals: non_terminals_order,
        terminals,
        productions,
        start,
    })
}

fn split_production_line(line: &str) -> Result<(String, String), GrammarError> {
    let (lhs, rhs) = if let Some(idx) = line.find("::=") {
        (&line[..idx], &line[idx + 3..])
    } else if let Some(idx) = line.find("->") {
        (&line[..idx], &line[idx + 2..])
    } else {
        return Err(GrammarError::ParseError(format!(
            "linha sem separador '->' ou '::=': '{line}'"
        )));
    };
    Ok((lhs.trim().to_string(), rhs.trim().to_string()))
}

fn is_valid_non_terminal_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => chars.next().is_none(),
        _ => false,
    }
}

fn parse_alternative(alt_text: &str) -> Result<Vec<Symbol>, GrammarError> {
    if alt_text == "&" || alt_text.is_empty() {
        return Ok(Vec::new());
    }

    let mut symbols = Vec::new();
    for c in alt_text.chars() {
        if c.is_ascii_lowercase() {
            symbols.push(Symbol::Terminal(c));
        } else if c.is_ascii_uppercase() {
            symbols.push(Symbol::NonTerminal(c.to_string()));
        } else {
            return Err(GrammarError::ParseError(format!(
                "símbolo inválido '{c}' em '{alt_text}' (use apenas a-z para terminais e A-Z para não-terminais)"
            )));
        }
    }
    Ok(symbols)
}
```

Add `mod grammar;` to `crates/grammar_engine/src/lib.rs`, below `mod symbol;`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (all `symbol::tests` plus 9 new `grammar::tests`).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/grammar.rs crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): add Grammar model and text parser"
```

---

### Task 3: Regularity validation

**Files:**
- Modify: `crates/grammar_engine/src/grammar.rs` (append function + tests to the file created in Task 2)
- Test: append to the existing `#[cfg(test)] mod tests` block in `grammar.rs`

**Interfaces:**
- Consumes: `Grammar`, `Symbol`, `GrammarError` (Tasks 1–2).
- Produces: `validate_regular(grammar: &Grammar) -> Result<(), GrammarError>`. Consumed by the UI later (Phase 1) and by this crate's own tests (Tasks 6–7) to assert built-in/derived grammars are regular.

- [ ] **Step 1: Write the failing tests**

Append these tests inside the existing `mod tests { ... }` block in `crates/grammar_engine/src/grammar.rs` (add them as new `#[test] fn` items alongside the ones from Task 2):

```rust
    #[test]
    fn accepts_right_linear_pdf_example() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        assert!(validate_regular(&grammar).is_ok());
    }

    #[test]
    fn accepts_multi_non_terminal_right_linear_grammar() {
        let grammar =
            parse_grammar("S -> aA | bB\nA -> bA | aC\nB -> aB | bC\nC -> a").unwrap();
        assert!(validate_regular(&grammar).is_ok());
    }

    #[test]
    fn rejects_non_terminal_in_the_middle_of_an_alternative() {
        let grammar = parse_grammar("S -> aAb\nA -> c").unwrap();
        let err = validate_regular(&grammar).unwrap_err();
        assert!(matches!(err, GrammarError::NotRegular { .. }));
    }

    #[test]
    fn rejects_more_than_one_non_terminal_in_an_alternative() {
        let grammar = parse_grammar("S -> AB\nA -> a\nB -> b").unwrap();
        let err = validate_regular(&grammar).unwrap_err();
        assert!(matches!(err, GrammarError::NotRegular { .. }));
    }
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `validate_regular` is not defined.

- [ ] **Step 3: Implement `validate_regular`**

Append this to `crates/grammar_engine/src/grammar.rs`, after the `parse_alternative` function and before the `#[cfg(test)]` block:

```rust
pub fn validate_regular(grammar: &Grammar) -> Result<(), GrammarError> {
    for non_terminal in &grammar.non_terminals {
        let alternatives = grammar.productions.get(non_terminal).into_iter().flatten();
        for alt in alternatives {
            for (index, symbol) in alt.iter().enumerate() {
                let is_last = index + 1 == alt.len();
                if matches!(symbol, Symbol::NonTerminal(_)) && !is_last {
                    return Err(GrammarError::NotRegular {
                        non_terminal: non_terminal.clone(),
                        alternative: format_alternative(alt),
                        reason: "não-terminal só pode aparecer como último símbolo (gramática regular à direita)".to_string(),
                    });
                }
            }
        }
    }
    Ok(())
}

fn format_alternative(alt: &[Symbol]) -> String {
    if alt.is_empty() {
        "&".to_string()
    } else {
        alt.iter().map(|s| s.to_string()).collect()
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (all previous tests plus 4 new ones).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/grammar.rs
git commit -m "feat(grammar_engine): validate that a grammar is right-linear (regular)"
```

---

### Task 4: `Stack` (the pilha)

**Files:**
- Create: `crates/grammar_engine/src/stack.rs`
- Modify: `crates/grammar_engine/src/lib.rs` (add `mod stack;`)
- Test: inline `#[cfg(test)] mod tests` in `stack.rs`

**Interfaces:**
- Consumes: `Symbol` (Task 1).
- Produces: `Stack` with `Stack::new() -> Self`, `Stack::is_empty(&self) -> bool`, `Stack::push_production(&mut self, rhs: &[Symbol])` (pushes so the **leftmost** symbol of `rhs` ends on top, per the PDF algorithm), `Stack::pop(&mut self) -> Option<Symbol>`, `Stack::snapshot_top_first(&self) -> Vec<Symbol>` (for UI display, top of stack first). Consumed by Task 5.

- [ ] **Step 1: Write the failing tests**

Create `crates/grammar_engine/src/stack.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn new_stack_is_empty() {
        assert!(Stack::new().is_empty());
    }

    #[test]
    fn push_production_puts_leftmost_symbol_on_top() {
        let mut stack = Stack::new();
        stack.push_production(&[Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]);
        assert_eq!(stack.pop(), Some(Symbol::Terminal('a')));
        assert_eq!(stack.pop(), Some(Symbol::NonTerminal("S".to_string())));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn snapshot_lists_top_of_stack_first() {
        let mut stack = Stack::new();
        stack.push_production(&[Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]);
        assert_eq!(
            stack.snapshot_top_first(),
            vec![Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]
        );
    }

    #[test]
    fn empty_production_pushes_nothing() {
        let mut stack = Stack::new();
        stack.push_production(&[]);
        assert!(stack.is_empty());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `Stack` is not defined.

- [ ] **Step 3: Implement `Stack`**

Prepend this to `crates/grammar_engine/src/stack.rs`, above `#[cfg(test)]`:

```rust
use crate::symbol::Symbol;

#[derive(Debug, Default)]
pub struct Stack {
    items: Vec<Symbol>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Pushes `rhs` so that its leftmost symbol ends up on top of the
    /// stack — matching the professor's algorithm ("o símbolo mais a
    /// esquerda da produção deve estar no topo da pilha").
    pub fn push_production(&mut self, rhs: &[Symbol]) {
        for symbol in rhs.iter().rev() {
            self.items.push(symbol.clone());
        }
    }

    pub fn pop(&mut self) -> Option<Symbol> {
        self.items.pop()
    }

    pub fn snapshot_top_first(&self) -> Vec<Symbol> {
        self.items.iter().rev().cloned().collect()
    }
}
```

Add `mod stack;` to `crates/grammar_engine/src/lib.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (all previous tests plus 4 new `stack::tests`).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/stack.rs crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): add the stack used by the derivation algorithm"
```

---

### Task 5: Random derivation via the stack

**Files:**
- Create: `crates/grammar_engine/src/derivation.rs`
- Modify: `crates/grammar_engine/src/lib.rs` (add `mod derivation;`)
- Test: inline `#[cfg(test)] mod tests` in `derivation.rs`

**Interfaces:**
- Consumes: `Grammar::alternatives` (Task 2), `Stack` (Task 4), `Symbol`, `GrammarError` (Task 1).
- Produces: `DerivationStep { non_terminal: String, production: Vec<Symbol>, stack_after: Vec<Symbol>, output_so_far: String }` (derives `Debug, Clone`), `Derivation { steps: Vec<DerivationStep>, sentence: String }` (derives `Debug, Clone`), `derive_random(grammar: &Grammar) -> Result<Derivation, GrammarError>`. Consumed by Tasks 6 and 7 (and later by the UI).

- [ ] **Step 1: Write the failing tests**

Create `crates/grammar_engine/src/derivation.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::parse_grammar;
    use crate::symbol::GrammarError;
    use regex::Regex;

    #[test]
    fn derives_a_sentence_matching_the_pdf_example_language() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let pattern = Regex::new(r"^a*ab$").unwrap();
        for _ in 0..50 {
            let derivation = derive_random(&grammar).unwrap();
            assert!(
                pattern.is_match(&derivation.sentence),
                "sentença '{}' não bate com a*ab",
                derivation.sentence
            );
        }
    }

    #[test]
    fn records_one_step_per_non_terminal_expansion() {
        let grammar = parse_grammar("S -> a").unwrap();
        let derivation = derive_random(&grammar).unwrap();
        assert_eq!(derivation.steps.len(), 1);
        assert_eq!(derivation.sentence, "a");
        assert_eq!(derivation.steps[0].non_terminal, "S");
        assert_eq!(derivation.steps[0].output_so_far, "a");
        assert!(derivation.steps[0].stack_after.is_empty());
    }

    #[test]
    fn errors_out_instead_of_looping_forever_on_a_dead_end_grammar() {
        let grammar = parse_grammar("S -> aS").unwrap();
        let result = derive_random(&grammar);
        assert_eq!(result.unwrap_err(), GrammarError::DerivationTooLong);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `derive_random` is not defined.

- [ ] **Step 3: Implement `derive_random`**

Prepend this to `crates/grammar_engine/src/derivation.rs`, above `#[cfg(test)]`:

```rust
use rand::Rng;

use crate::grammar::Grammar;
use crate::stack::Stack;
use crate::symbol::{GrammarError, Symbol};

const MAX_DERIVATION_STEPS: usize = 10_000;

#[derive(Debug, Clone)]
pub struct DerivationStep {
    pub non_terminal: String,
    pub production: Vec<Symbol>,
    pub stack_after: Vec<Symbol>,
    pub output_so_far: String,
}

#[derive(Debug, Clone)]
pub struct Derivation {
    pub steps: Vec<DerivationStep>,
    pub sentence: String,
}

/// Implements the professor's algorithm exactly: pick a production for the
/// current non-terminal, push it (leftmost symbol on top), then while the
/// stack isn't empty pop — terminals go to the output, non-terminals get
/// expanded again from step one.
pub fn derive_random(grammar: &Grammar) -> Result<Derivation, GrammarError> {
    let mut rng = rand::thread_rng();
    let mut stack = Stack::new();
    let mut output = String::new();
    let mut steps = Vec::new();

    stack.push_production(&[Symbol::NonTerminal(grammar.start.clone())]);

    let mut guard = 0usize;
    while let Some(symbol) = stack.pop() {
        guard += 1;
        if guard > MAX_DERIVATION_STEPS {
            return Err(GrammarError::DerivationTooLong);
        }

        match symbol {
            Symbol::Terminal(c) => output.push(c),
            Symbol::NonTerminal(name) => {
                let alternatives = grammar
                    .alternatives(&name)
                    .ok_or_else(|| GrammarError::UndefinedNonTerminal(name.clone()))?;
                let choice = rng.gen_range(0..alternatives.len());
                let production = alternatives[choice].clone();
                stack.push_production(&production);
                steps.push(DerivationStep {
                    non_terminal: name,
                    production,
                    stack_after: stack.snapshot_top_first(),
                    output_so_far: output.clone(),
                });
            }
        }
    }

    Ok(Derivation {
        steps,
        sentence: output,
    })
}
```

Add `mod derivation;` to `crates/grammar_engine/src/lib.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (all previous tests plus 3 new `derivation::tests`).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/derivation.rs crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): derive random sentences with the stack algorithm"
```

---

### Task 6: Grammar → regular expression conversion

**Files:**
- Create: `crates/grammar_engine/src/regex_conversion.rs`
- Modify: `crates/grammar_engine/src/lib.rs` (add `mod regex_conversion;`)
- Test: inline `#[cfg(test)] mod tests` in `regex_conversion.rs`

**Interfaces:**
- Consumes: `Grammar::alternatives`, `Grammar.non_terminals`, `Grammar.start` (Task 2), `Symbol`, `GrammarError` (Task 1); tests also consume `parse_grammar` (Task 2) and `derive_random` (Task 5).
- Produces: `to_regex(grammar: &Grammar) -> Result<String, GrammarError>`. Consumed by Task 7 and later by the UI.

Module name note: this file is **not** called `regex.rs` on purpose — that name would collide with the external `regex` crate used in dev-dependencies, since both would be reachable as `regex::...` from this crate.

- [ ] **Step 1: Write the failing tests**

Create `crates/grammar_engine/src/regex_conversion.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::derive_random;
    use crate::grammar::parse_grammar;
    use regex::Regex;

    #[test]
    fn matches_the_exact_pdf_example_output() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        assert_eq!(to_regex(&grammar).unwrap(), "a*ab");
    }

    #[test]
    fn handles_a_chain_of_two_non_terminals() {
        let grammar = parse_grammar("S -> aA\nA -> bA | c").unwrap();
        assert_eq!(to_regex(&grammar).unwrap(), "ab*c");
    }

    #[test]
    fn unions_multiple_self_referencing_alternatives_before_starring() {
        let grammar = parse_grammar("S -> aS | bS | c").unwrap();
        let pattern_text = to_regex(&grammar).unwrap();
        let pattern = Regex::new(&format!("^{pattern_text}$")).unwrap();
        assert!(pattern.is_match("c"));
        assert!(pattern.is_match("abababc"));
        assert!(!pattern.is_match("d"));
    }

    #[test]
    fn generated_sentences_always_match_the_derived_regex() {
        let grammar =
            parse_grammar("S -> aA | bB\nA -> bA | aC\nB -> aB | bC\nC -> a").unwrap();
        let pattern_text = to_regex(&grammar).unwrap();
        let pattern = Regex::new(&format!("^{pattern_text}$")).unwrap();
        for _ in 0..100 {
            let derivation = derive_random(&grammar).unwrap();
            assert!(
                pattern.is_match(&derivation.sentence),
                "sentença '{}' não bate com {pattern_text}",
                derivation.sentence
            );
        }
    }
}
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `to_regex` is not defined.

- [ ] **Step 3: Implement `to_regex`**

Prepend this to `crates/grammar_engine/src/regex_conversion.rs`, above `#[cfg(test)]`:

```rust
use std::collections::HashMap;

use crate::grammar::Grammar;
use crate::symbol::{GrammarError, Symbol};

#[derive(Debug, Clone)]
struct Term {
    coeff: String,
    target: Option<String>,
}

type Equation = Vec<Term>;

/// Converts a regular grammar into an equivalent regular expression using
/// the classic non-terminal elimination method (Arden's rule applied one
/// non-terminal at a time: `X = aX + b  =>  X = a*b`). Output uses
/// standard regex syntax (see Global Constraints in the plan this
/// implements) so it can be fed straight into the `regex` crate.
pub fn to_regex(grammar: &Grammar) -> Result<String, GrammarError> {
    let mut equations: HashMap<String, Equation> = grammar
        .non_terminals
        .iter()
        .map(|nt| (nt.clone(), build_equation(grammar, nt)))
        .collect();

    let non_start: Vec<String> = grammar
        .non_terminals
        .iter()
        .filter(|nt| **nt != grammar.start)
        .cloned()
        .collect();

    for eliminated in non_start {
        let equation = equations.remove(&eliminated).ok_or_else(|| {
            GrammarError::RegexConversionFailed(format!(
                "equação de '{eliminated}' não encontrada durante a eliminação"
            ))
        })?;
        let resolved = resolve_self_reference(&eliminated, equation);

        for equation in equations.values_mut() {
            substitute(equation, &eliminated, &resolved);
        }
    }

    let start_equation = equations.remove(&grammar.start).ok_or_else(|| {
        GrammarError::RegexConversionFailed(
            "equação do símbolo inicial não encontrada".to_string(),
        )
    })?;
    let final_terms = resolve_self_reference(&grammar.start, start_equation);

    let mut pieces = Vec::with_capacity(final_terms.len());
    for term in final_terms {
        if term.target.is_some() {
            return Err(GrammarError::RegexConversionFailed(
                "gramática tem não-terminais que não puderam ser eliminados".to_string(),
            ));
        }
        pieces.push(term.coeff);
    }

    Ok(regex_union(&pieces))
}

fn build_equation(grammar: &Grammar, non_terminal: &str) -> Equation {
    grammar
        .alternatives(non_terminal)
        .into_iter()
        .flatten()
        .map(|alt| alternative_to_term(alt))
        .collect()
}

fn alternative_to_term(alt: &[Symbol]) -> Term {
    let mut coeff = String::new();
    let mut target = None;
    for symbol in alt {
        match symbol {
            Symbol::Terminal(c) => coeff.push(*c),
            Symbol::NonTerminal(name) => target = Some(name.clone()),
        }
    }
    Term { coeff, target }
}

/// Applies Arden's rule to remove `non_terminal` from its own equation,
/// distributing the resulting `alpha*` prefix over every remaining term.
fn resolve_self_reference(non_terminal: &str, equation: Equation) -> Equation {
    let (self_terms, mut other_terms): (Vec<Term>, Vec<Term>) = equation
        .into_iter()
        .partition(|term| term.target.as_deref() == Some(non_terminal));

    if self_terms.is_empty() {
        return other_terms;
    }

    let alpha = regex_union(
        &self_terms
            .into_iter()
            .map(|t| t.coeff)
            .collect::<Vec<_>>(),
    );
    let alpha_star = kleene_star(&alpha);

    for term in &mut other_terms {
        term.coeff = format!("{alpha_star}{}", term.coeff);
    }

    other_terms
}

fn substitute(equation: &mut Equation, eliminated: &str, resolved: &Equation) {
    let mut new_terms = Vec::new();
    for term in equation.drain(..) {
        if term.target.as_deref() == Some(eliminated) {
            for resolved_term in resolved {
                new_terms.push(Term {
                    coeff: format!("{}{}", term.coeff, resolved_term.coeff),
                    target: resolved_term.target.clone(),
                });
            }
        } else {
            new_terms.push(term);
        }
    }
    *equation = new_terms;
}

fn regex_union(parts: &[String]) -> String {
    match parts.len() {
        0 => String::new(),
        1 => parts[0].clone(),
        _ => format!("({})", parts.join("|")),
    }
}

fn kleene_star(alpha: &str) -> String {
    if alpha.is_empty() {
        String::new()
    } else if alpha.chars().count() == 1 {
        format!("{alpha}*")
    } else {
        format!("({alpha})*")
    }
}
```

Add `mod regex_conversion;` to `crates/grammar_engine/src/lib.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (all previous tests plus 4 new `regex_conversion::tests`).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/regex_conversion.rs crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): convert regular grammars to regular expressions"
```

---

### Task 7: Three built-in example grammars

**Files:**
- Create: `crates/grammar_engine/src/examples.rs`
- Modify: `crates/grammar_engine/src/lib.rs` (add `mod examples;`)
- Test: inline `#[cfg(test)] mod tests` in `examples.rs`

**Interfaces:**
- Consumes: `parse_grammar` (Task 2), `validate_regular` (Task 3), `derive_random` (Task 5), `to_regex` (Task 6).
- Produces: `ExampleGrammar { name: &'static str, description: &'static str, source: &'static str }` (derives `Debug, Clone, Copy`), `EXAMPLE_SOURCES: [ExampleGrammar; 3]`, `examples() -> Vec<(&'static ExampleGrammar, Grammar)>`. Satisfies the assignment's "três gramáticas de exemplo" requirement; consumed by the UI in Phase 1.

- [ ] **Step 1: Write the failing tests**

Create `crates/grammar_engine/src/examples.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::derive_random;
    use crate::grammar::validate_regular;
    use crate::regex_conversion::to_regex;
    use regex::Regex;

    #[test]
    fn ships_exactly_three_examples() {
        assert_eq!(EXAMPLE_SOURCES.len(), 3);
    }

    #[test]
    fn every_example_parses_and_is_a_valid_regular_grammar() {
        for (example, grammar) in examples() {
            assert!(
                validate_regular(&grammar).is_ok(),
                "exemplo '{}' não é uma gramática regular válida",
                example.name
            );
        }
    }

    #[test]
    fn every_example_can_derive_a_sentence_matching_its_own_regex() {
        for (example, grammar) in examples() {
            let pattern_text = to_regex(&grammar).unwrap();
            let pattern = Regex::new(&format!("^{pattern_text}$")).unwrap();
            let derivation = derive_random(&grammar).unwrap();
            assert!(
                pattern.is_match(&derivation.sentence),
                "exemplo '{}': sentença '{}' não bate com {pattern_text}",
                example.name,
                derivation.sentence
            );
        }
    }
}
```

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL — `EXAMPLE_SOURCES` / `examples` are not defined.

- [ ] **Step 3: Implement the example grammars**

Prepend this to `crates/grammar_engine/src/examples.rs`, above `#[cfg(test)]`:

```rust
use crate::grammar::{parse_grammar, Grammar};

#[derive(Debug, Clone, Copy)]
pub struct ExampleGrammar {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
}

pub const EXAMPLE_SOURCES: [ExampleGrammar; 3] = [
    ExampleGrammar {
        name: "Exemplo do professor",
        description: "S -> aS | ab — o exemplo do slide da disciplina.",
        source: "S -> aS | ab",
    },
    ExampleGrammar {
        name: "Cadeia a-b-c",
        description: "Uma sequência de a's, seguida de b's, terminando em c.",
        source: "S -> aS | aA\nA -> bA | c",
    },
    ExampleGrammar {
        name: "Labirinto de bifurcações",
        description: "Três não-terminais interligados — pensado para o modo labirinto.",
        source: "S -> aA | bB\nA -> bA | aC\nB -> aB | bC\nC -> a",
    },
];

/// Parses the three built-in grammars. Panics only if one of the hardcoded
/// `source` strings above is itself malformed — a programmer error caught
/// immediately by this module's own tests, never a possibility from user
/// input (see Global Constraints in the plan this implements).
pub fn examples() -> Vec<(&'static ExampleGrammar, Grammar)> {
    EXAMPLE_SOURCES
        .iter()
        .map(|example| {
            let grammar = parse_grammar(example.source).unwrap_or_else(|err| {
                panic!(
                    "gramática de exemplo embutida '{}' é inválida: {err}",
                    example.name
                )
            });
            (example, grammar)
        })
        .collect()
}
```

Add `mod examples;` to `crates/grammar_engine/src/lib.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p grammar_engine`
Expected: PASS (all previous tests plus 3 new `examples::tests`).

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/examples.rs crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): ship three built-in example grammars"
```

---

### Task 8: Public API surface and full verification

**Files:**
- Modify: `crates/grammar_engine/src/lib.rs` (replace the `mod`-only content built up across Tasks 1–7 with the final version below, adding `pub use` re-exports)

**Interfaces:**
- Consumes: everything produced by Tasks 1–7.
- Produces: the crate's public API, as used by `grammar_quest` from Phase 1 onward: `Symbol`, `GrammarError`, `Grammar`, `parse_grammar`, `validate_regular`, `Stack`, `Derivation`, `DerivationStep`, `derive_random`, `to_regex`, `ExampleGrammar`, `EXAMPLE_SOURCES`, `examples`.

- [ ] **Step 1: Write the failing test**

Replace the entire content of `crates/grammar_engine/src/lib.rs` with:

```rust
//! Motor de gramáticas regulares: parsing, derivação via pilha e conversão
//! para expressão regular. Ver docs/superpowers/specs/2026-09-14-grammar-quest-design.md.

mod derivation;
mod examples;
mod grammar;
mod regex_conversion;
mod stack;
mod symbol;

pub use derivation::{derive_random, Derivation, DerivationStep};
pub use examples::{examples, ExampleGrammar, EXAMPLE_SOURCES};
pub use grammar::{parse_grammar, validate_regular, Grammar};
pub use regex_conversion::to_regex;
pub use stack::Stack;
pub use symbol::{GrammarError, Symbol};

#[cfg(test)]
mod public_api_tests {
    use super::*;

    #[test]
    fn end_to_end_public_api_reproduces_the_pdf_example() {
        let grammar = parse_grammar("S -> aS | ab").expect("grammar text is valid");
        validate_regular(&grammar).expect("grammar is regular");
        let derivation = derive_random(&grammar).expect("grammar is productive");
        assert!(!derivation.sentence.is_empty());
        assert_eq!(to_regex(&grammar).expect("grammar converts"), "a*ab");
        assert_eq!(examples().len(), 3);
    }
}
```

(This test module is allowed to use `expect()` — it lives under `#[cfg(test)]`, matching the Global Constraints.)

- [ ] **Step 2: Run tests to verify they fail to compile**

Run: `cargo test -p grammar_engine`
Expected: FAIL if any re-export name in `lib.rs` doesn't match what Tasks 1–7 actually defined (this step is a consistency check across the whole crate, not just this new test).

- [ ] **Step 3: Fix any mismatches**

If Step 2 fails, the error will point at the exact missing/misnamed item — fix the `pub use` line (or the underlying definition) to match. No new logic should be needed; every type and function above was already implemented in Tasks 1–7.

- [ ] **Step 4: Run full verification**

Run, in order, from `/Users/gustavoconstante/Projects/Grammar Quest`:

```bash
cargo test -p grammar_engine
cargo fmt -p grammar_engine -- --check
cargo clippy -p grammar_engine --all-targets -- -D warnings
cargo build
```

Expected: all four commands succeed. If `cargo fmt --check` fails, run `cargo fmt -p grammar_engine` (no `--check`) to apply formatting, then re-run the check. If `clippy` reports warnings, fix them in the relevant module from Tasks 1–7 before proceeding — do not silence with `#[allow(...)]` unless the lint is a false positive you can justify in a one-line comment.

- [ ] **Step 5: Commit**

```bash
cd "/Users/gustavoconstante/Projects/Grammar Quest"
git add crates/grammar_engine/src/lib.rs
git commit -m "feat(grammar_engine): expose public API, close out Fase 0"
```

---

## After this plan

Update `ROADMAP.md`, checking off every completed item under "Fase 0 — Motor de gramática", before starting Fase 1 (the egui/macroquad panel). Fase 1 needs its own plan — it touches `grammar_quest`, a different crate with different constraints (UI, not pure logic).

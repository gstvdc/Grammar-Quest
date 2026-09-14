# Grammar Quest

Trabalho de Desenvolvimento (TD1) — Linguagens Formais e Autômatos
(UNESC). Um labirinto onde cada bifurcação é uma produção de uma
gramática regular: o jogador deriva a sentença andando pelo mapa,
enquanto a pilha, a derivação passo a passo e a expressão regular
equivalente são mostradas ao vivo.

Requisitos do trabalho e arquitetura: ver `CLAUDE.md`.
Fases de desenvolvimento: ver `ROADMAP.md`.
Design detalhado: ver `docs/superpowers/specs/2026-09-14-grammar-quest-design.md`.

## Rodar

```bash
cargo run -p grammar_quest
```

## Testar o motor de gramática

```bash
cargo test -p grammar_engine
```

## Estrutura

```
crates/
  grammar_engine/   # gramática, pilha, derivação, conversão para regex
  grammar_quest/    # jogo (macroquad + egui-macroquad)
context/             # enunciado do trabalho (PDF)
docs/superpowers/specs/  # spec de design
```

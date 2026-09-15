# Grammar Quest

Trabalho de Desenvolvimento (TD1) da disciplina Linguagens Formais e
Autômatos (UNESC, prof. André Faria Ruaro) — ver `context/Linguagens Formais
- Aula 6 - TD01.pdf`. Implementado como um jogo em Rust: **Grammar Maze**.

## O que o trabalho exige (não negociável)

1. Entrada de uma gramática `G = {N, T, P, S}`.
2. Geração de sentenças **aleatórias**.
3. Mecanismo de **derivação implementado com pilha** — algoritmo exato do
   PDF (escolher produção → empilhar com símbolo mais à esquerda no topo →
   desempilhar: terminal vai pra saída, não-terminal expande de novo).
4. Apenas **gramáticas regulares**.
5. **Interface gráfica** com campos de entrada e exibição do resultado da
   derivação.
6. **Três gramáticas de exemplo** disponíveis para seleção dentro do
   próprio software.
7. Conversão da gramática regular para **expressão regular** (eliminação de
   variáveis não-terminais) após a derivação.

Qualquer mudança no jogo não pode quebrar nenhum desses pontos — eles são
avaliados independentemente da camada de jogo.

## Arquitetura

Workspace Cargo com duas crates:

- `crates/grammar_engine` — lib pura, **sem dependência de UI/jogo**.
  Parsing da gramática, validação de regularidade, derivação via pilha,
  conversão para regex, geração de sentenças de exemplo/distratoras.
  Deve ser testável isoladamente (`cargo test -p grammar_engine`).
- `crates/grammar_quest` — binário do jogo (macroquad + egui-macroquad).
  Consome `grammar_engine` via dependência de path; não deve reimplementar
  lógica de gramática.

O painel de UI (pilha / derivação / regex) construído na Fase 1 é
reaproveitado dentro do labirinto na Fase 2 — não duplicar essa lógica de
apresentação entre telas.

Ver `ROADMAP.md` para as fases e `docs/superpowers/specs/` para a spec de
design completa.

## Comandos

```bash
cargo build                        # build de todo o workspace
cargo run -p grammar_quest         # roda o jogo
cargo test -p grammar_engine       # testes do motor (fonte de verdade dos requisitos 1-4 e 7)
cargo fmt && cargo clippy --workspace --all-targets
```

## Convenções

- Edition 2024, `rand = "0.8"` fixado nas duas crates (evitar duas versões
  de `rand` no grafo de dependências).
- Sem `unwrap()`/`expect()` em código de produção do `grammar_engine` fora
  de testes — entrada de gramática vem de UI e pode ser inválida; erros
  devem virar `Result` tratável pela UI, não panics.
- Sem comentários explicando o óbvio; comentar só decisões não-óbvias
  (ex.: por que a pilha usa `Vec<Symbol>` e não `VecDeque`).
- Não adicionar dependências novas sem necessidade concreta (YAGNI) — em
  especial nada que auto-modifique `settings.json`/hooks ou baixe
  pacotes/MCPs não verificados.

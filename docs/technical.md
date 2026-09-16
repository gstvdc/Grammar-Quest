# Documentação técnica

## Estrutura

O workspace Cargo tem duas crates:

- `grammar_engine`: biblioteca pura, sem dependência de interface. Faz parsing,
  validação de regularidade, derivação por pilha, geração aleatória, exemplos,
  distratores e conversão para expressão regular.
- `grammar_quest`: aplicativo desktop em `macroquad` com sobreposição
  `egui-macroquad`. Consome exclusivamente a API de `grammar_engine` para toda
  lógica formal.

## Fluxo da derivação

`DerivationState` mantém a pilha e aplica uma produção escolhida para o
não-terminal no topo. A produção é empilhada em ordem reversa, deixando o
símbolo mais à esquerda no topo. Terminais são desempilhados e acrescentados à
saída; não-terminais solicitam uma nova produção. O histórico de passos e
eventos é reutilizado tanto no laboratório quanto no labirinto.

## Interface e jogo

- `ui/laboratory.rs`: exemplos, criação de gramática, geração e dificuldade.
- `ui/side_panel.rs`: apresentação compartilhada de pilha, derivação e regex.
- `ui/game_hud.rs`: HUD e pilha durante o jogo.
- `maze.rs` e `gameplay.rs`: sala, portas, colisão e aplicação das escolhas.
- `state.rs`: transições entre menu, laboratório, jogo e vitória.

## Testes

`grammar_engine` é a fonte de verdade para parsing, regularidade, pilha,
derivação e regex. Os testes do aplicativo verificam transições, dificuldade,
pilha, HUD e geometria do labirinto.

```bash
cargo test -p grammar_engine
cargo test -p grammar_quest
```

## Pacotes de distribuição

Os scripts em `scripts/` geram o bundle macOS e o DMG. O executável Windows
x64 é compilado com:

```bash
cargo build --release -p grammar_quest --target x86_64-pc-windows-gnu
```

O repositório inclui as atribuições dos assets e a licença da fonte Press Start
2P em `crates/grammar_quest/assets/`.

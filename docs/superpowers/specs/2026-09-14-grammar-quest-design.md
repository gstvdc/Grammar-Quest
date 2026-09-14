# Grammar Quest — Design

Data: 2026-09-14
Status: aprovado (escopo do jogo confirmado pelo usuário: apenas Grammar Maze)

## Contexto

Trabalho de Desenvolvimento (TD1) de Linguagens Formais e Autômatos
(UNESC). Requisitos completos em `context/Linguagens Formais - Aula 6 -
TD01.pdf` e resumidos em `CLAUDE.md`. A entrega exige código-fonte +
três gramáticas de exemplo selecionáveis pelo próprio software.

Decisão de produto: em vez de um "gerador de sentenças com UI", o
trabalho é apresentado como um jogo — **Grammar Maze** — onde a mecânica
central (labirinto com bifurcações) é uma representação direta do
algoritmo de derivação exigido pelo professor. Cada escolha de corredor
é literalmente uma escolha de produção da gramática.

## Objetivo

Construir, em Rust, um programa com interface gráfica que:
1. Recebe/seleciona uma gramática regular `G = {N, T, P, S}`.
2. Deriva sentenças aleatoriamente usando uma pilha.
3. Exibe pilha, derivação passo a passo, sentença e expressão regular
   resultante.
4. Apresenta essa derivação como um labirinto jogável.

## Approach escolhido

Workspace Cargo com duas crates, decisão já tomada e implementada:

- **`grammar_engine`** (lib): toda a lógica de linguagens formais. Sem
  dependência de macroquad/egui. Isso permite testar o núcleo (que é a
  parte avaliada academicamente) sem precisar de um contexto gráfico, e
  mantém o motor reutilizável caso o jogo mude de engine no futuro.
- **`grammar_quest`** (bin): macroquad (janela, input, desenho 2D) +
  egui-macroquad (formulários, painéis de texto). Consome o motor.

Alternativas consideradas e descartadas:
- **Tudo em um único crate/binário**: mais simples de iniciar, mas
  mistura lógica de gramática com estado de jogo (input, câmera, frames)
  — dificulta testar o motor isoladamente, que é justamente a parte que
  o professor avalia com mais rigor. Descartado.
- **egui puro (eframe) sem macroquad**: mais simples para formulários,
  mas desenhar um labirinto 2D fluido com tiles/animação é natural em
  macroquad e artificial em egui (que é um immediate-mode UI toolkit, não
  uma engine 2D). Descartado — perderíamos o "fluido e bonito" pedido.

## Fluxo de dados

```
Gramática (texto ou preset)
        │
        ▼
  grammar_engine::parse()
        │
        ▼
  validate_regular()  ──► erro de validação exibido na UI, não panic
        │
        ▼
  derive_random()  (usa Vec<Symbol> como pilha)
        │      cada passo emite um DerivationStep { rule_applied, stack_snapshot, output_so_far }
        ▼
  Sentence (String) + Vec<DerivationStep>
        │
        ▼
  to_regex()  (eliminação de variáveis não-terminais)
        │
        ▼
  UI: painel lateral (pilha/derivação/regex) + labirinto (Fase 2)
```

No modo labirinto, o "próximo passo" não é sorteado automaticamente:
o jogador escolhe o corredor, e essa escolha É a produção aplicada.
Internamente isso chama a mesma função de aplicar-uma-produção que o
modo "gerar aleatório" usa para sortear — só muda quem decide qual
produção usar (RNG vs. input do jogador).

## Componentes

### `grammar_engine`

- `Symbol` — enum `Terminal(char)` | `NonTerminal(String)`.
- `Production` — lado esquerdo (`NonTerminal`) + lado direito
  (`Vec<Symbol>`), restrito à forma regular (`A -> aB`, `A -> a`, `A ->
  ε`) na validação.
- `Grammar` — `non_terminals`, `terminals`, `productions:
  HashMap<NonTerminal, Vec<Vec<Symbol>>>`, `start: NonTerminal`.
- `Stack` — wrapper fino sobre `Vec<Symbol>` com `push_production` (empurra
  símbolos na ordem exigida pelo PDF: símbolo mais à esquerda no topo) e
  `pop`.
- `derive_random(&Grammar) -> Derivation` — loop do algoritmo do PDF.
- `derive_step(&Grammar, state: &mut DerivationState, choice: usize)` —
  versão orientada a jogador: aplica UMA produção escolhida e retorna o
  novo estado (pilha, saída parcial). Usada pelo labirinto.
- `to_regex(&Grammar) -> String` — eliminação de variáveis.
- `examples() -> [Grammar; 3]` — as três gramáticas de exemplo exigidas.

Contrato de cada unidade: dado uma `Grammar` válida, as funções nunca
entram em pânico; entrada inválida vira `Result<_, GrammarError>` com
mensagem legível para a UI.

### `grammar_quest`

- `screens/menu.rs` — seleção de gramática (preset ou editor).
- `screens/maze.rs` — renderização do labirinto + input do jogador +
  chamada a `derive_step`.
- `ui/side_panel.rs` — componente egui reaproveitado entre Fase 1 e Fase
  2: pilha, derivação, regex. Não conhece macroquad, só recebe o estado
  do `grammar_engine` e desenha.
- `state.rs` — `GameState` enum (Menu, Playing, Won), guarda a
  `Grammar` ativa e o `DerivationState` corrente.

## Tratamento de erros

- Gramática mal formada no editor → `GrammarError` exibido inline no
  formulário, sem travar a UI.
- Gramática que não é regular → mesma via, mensagem específica
  ("produção X não é regular").
- Nunca usar `unwrap()`/`expect()` fora de testes no `grammar_engine`.

## Testes

- `grammar_engine`: testes unitários por função pública, incluindo o
  caso exato do PDF (`S ::= aS | ab`, verificar que toda sentença
  derivada bate com a regex `a*ab` via crate `regex` em dev-dependency).
- `grammar_quest`: sem testes automatizados de renderização (não vale a
  pena para um jogo pequeno); verificação manual rodando `cargo run -p
  grammar_quest` a cada fase, conforme o skill `run`/verificação antes de
  reportar como concluído.

## Fora de escopo

Ver seção "Fora de escopo" do `ROADMAP.md`.

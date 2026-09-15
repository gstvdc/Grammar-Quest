# Checklist de aceite (TD1)

Roteiro reproduzível em uma máquina limpa, ligando cada um dos sete
requisitos não-negociáveis de `CLAUDE.md` (a mesma lista de
`context/td01-linguagens-formais.pdf`) a uma ação concreta na UI e
ao resultado observável esperado. Use isto para apresentar o trabalho ou para
validar uma sessão antes de entregar.

## Pré-requisitos

- Rust estável com suporte a edition 2024. Verificado nesta máquina com
  `rustc 1.98.0 (88d9e12ae 2026-08-18)` (`rustc --version`); qualquer stable
  mais recente também serve.
- macOS: ver a nota de compatibilidade no fim deste documento antes de rodar
  `cargo run -p grammar_quest` em automação/CI sem sessão gráfica.

```bash
git clone <repo>
cd "Grammar Quest"
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p grammar_quest
```

## Roteiro por requisito

### 1. Entrada de uma gramática `G = {N, T, P, S}`

- Abra o app; a tela **Laboratório** já mostra o preset "Exemplo do
  professor" (`S -> aS | ab`) carregado no editor de produções.
- Observe a ficha **`G = {N, T, P, S}`** logo abaixo do editor
  (`show_grammar_overview_card`, `crates/grammar_quest/src/ui/editor.rs`).
- **Esperado:** `N = {S}`, `T = {a,b}`, `S = S`, duas linhas em `P`
  (`S -> aS`, `S -> ab`) e o texto "2 produção(ões) no total".
- Edite o texto para `S aS` (sem `->`, sintaxe inválida).
- **Esperado:** a ficha muda para "Gramática inválida — corrija a sintaxe
  para ver N/T/P/S." — nenhum dos quatro componentes é inventado a partir de
  uma entrada que não parseia.

### 2. Geração de sentenças aleatórias

- Com o preset padrão selecionado, clique **"⚡ Gerar sentença aleatória"**.
- **Esperado:** aparece uma sentença que casa com `a*ab` (por exemplo `aab`,
  `aaab`, `ab`); clicar várias vezes produz sentenças diferentes.

### 3. Mecanismo de derivação implementado com pilha

- Clique **"▶ JOGAR LABIRINTO LIVRE"** com o preset padrão.
- Cada porta na sala representa uma alternativa de produção do não-terminal
  atual no topo da pilha (`S -> aS` e `S -> ab`, no exemplo do professor).
  Ande até uma porta para aplicá-la.
- Pressione `Tab` para abrir o painel lateral e observe a pilha atual e o log
  de passos, atualizados a cada porta.
- **Esperado:** ao escolher `S -> aS` a pilha mostra `a` no topo empilhado à
  esquerda, com `S` disponível para nova expansão; ao finalmente escolher
  `S -> ab`, a pilha esvazia e a porta de saída (★) aparece.
- Para o traço formal completo do algoritmo do PDF: no Laboratório, clique
  "Gerar sentença aleatória" repetidamente até obter a sequência de escolhas
  1/1/2 (ou rode `cargo test -p grammar_engine derivation` e veja o teste
  `records_a_full_event_trace_matching_the_pdf_fixture`, que reproduz exatamente
  o fixture do PDF: escolhas 1/1/2, pops `a/a/a/b`, saída `aaab`).

### 4. Apenas gramáticas regulares

- No editor, troque o texto para uma gramática livre de contexto simples:
  `S -> AB\nA -> a\nB -> b`.
- **Esperado:** a tira de telemetria no topo do laboratório mostra
  "REGULAR: NÃO" em vermelho/rosa; tentar "JOGAR LABIRINTO LIVRE" falha com
  uma mensagem de erro visível (não entra em modo Jogando).
- Volte ao preset padrão; **esperado:** "REGULAR: SIM" em verde.
- Teste adicional de improdutividade: digite `S -> aB\nB -> aB` (regular,
  mas sem produção terminal para `B`). **Esperado:** "JOGAR LABIRINTO LIVRE"
  falha com erro visível e a tela permanece no Laboratório (não há como
  entrar em uma sessão de jogo sem sentença derivável) — ver
  `crates/grammar_quest/src/state.rs`, teste
  `start_free_maze_fails_on_an_unproductive_grammar`.

### 5. Interface gráfica com entrada e resultado da derivação

- Toda a interação acima (editor de produções, ficha `G={N,T,P,S}`, botão de
  geração, labirinto jogável, painel lateral com pilha/passos/regex) é uma
  única aplicação gráfica (`egui` sobre `macroquad`), sem etapas de linha de
  comando após `cargo run -p grammar_quest`.
- Controles: `WASD`/setas movem o personagem, `Tab` mostra/esconde o painel
  formal, `Esc` volta ao Laboratório (ou fecha o app, se já estiver lá).

### 6. Três gramáticas de exemplo no software

- No Laboratório, abra o combo **"EXEMPLOS PRONTOS"**.
- **Esperado:** exatamente três opções — "Exemplo do professor", "Cadeia
  a-b-c" e "Labirinto de bifurcações" — cada uma com uma descrição em
  itálico abaixo do combo ao ser selecionada (`crates/grammar_engine/src/examples.rs`,
  `EXAMPLE_SOURCES`). Selecionar qualquer uma recarrega o editor e a ficha
  `G={N,T,P,S}` correspondente; as três são jogáveis no labirinto.

### 7. Conversão da gramática regular para expressão regular após a derivação

- Com o preset padrão, clique "Gerar sentença aleatória" (ou jogue o
  labirinto até vencer).
- No painel lateral, expanda a seção recolhível **"Equações e eliminação de
  variáveis"** (`crates/grammar_quest/src/ui/side_panel.rs`).
- **Esperado, nessa ordem:** "Equações iniciais:" mostrando `S=aS+ab`;
  "Eliminação de variáveis (regra de Arden):" mostrando o passo resolvido
  `S=a*ab`; e a expressão final **`a*ab`** — não apenas o resultado, mas o
  caminho até ele.

## Verificação automatizada equivalente

Cada afirmação "esperado" acima tem um teste correspondente que roda em CI
local:

```bash
cargo test -p grammar_engine   # requisitos 1-4 e 7 (motor puro, sem UI)
cargo test -p grammar_quest    # requisitos 1, 3, 4, 6 (integração com estado/UI)
cargo clippy --workspace --all-targets -- -D warnings
```

## Limitações conhecidas

- **macOS sem sessão gráfica / identidade de app:** `cargo run -p
  grammar_quest` pode abortar dentro de `miniquad 0.4.11` ao inicializar o
  menu da aplicação quando `NSRunningApplication.localizedName` retorna nulo
  (visto em automação headless neste ambiente). Builds e testes não são
  afetados — apenas a janela gráfica interativa. Rode em uma sessão gráfica
  normal, ou construa o bundle com `scripts/build-macos-app.sh` e abra
  `dist/Grammar Quest.app`, para o smoke test visual.
- **T6/T7 fora deste checklist:** melhorias de análise de produtividade por
  ponto fixo (além do fallback de `derive_random`) e de responsividade de
  layout (viewport dinâmico) foram avaliadas e conscientemente adiadas por
  desproporcionais ao escopo desta disciplina — ver `ROADMAP.md` e
  `docs/tasks/2026-09-15-audit-adjustments.md` (T6, T7).

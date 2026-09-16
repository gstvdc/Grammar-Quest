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

- Na tela inicial, escolha **JOGAR**, depois **MODO LIVRE**.
- Em **PREPARAÇÃO LIVRE**, clique em **CRIAR GRAMÁTICA** e informe as
  produções no campo de texto, por exemplo `S -> aS | ab`.
- Clique em **SALVAR E VOLTAR**. A aplicação analisa o texto como
  `G = {N, T, P, S}` antes de liberar a geração ou o labirinto.
- **Esperado:** para `S -> aS | ab`, a ficha do resultado mostra
  `N = {S}`, `T = {a,b}`, `S = S` e duas produções em `P`.

### 2. Geração de sentenças aleatórias

- Em **PREPARAÇÃO LIVRE**, com **Trilha Inicial** selecionada, clique
  **GERAR SENTENÇA**.
- **Esperado:** aparece uma sentença que casa com `a*ab` (por exemplo `aab`,
  `aaab`, `ab`); clicar várias vezes produz sentenças diferentes.

### 3. Mecanismo de derivação implementado com pilha

- Em **PREPARAÇÃO LIVRE**, clique **▶ JOGAR LABIRINTO** com a gramática
  **Trilha Inicial** ativa.
- Cada porta na sala representa uma alternativa de produção do não-terminal
  atual no topo da pilha (`S -> aS` e `S -> ab`, na Trilha Inicial).
  Ande até uma porta para aplicá-la.
- Observe a pilha no rodapé da coluna **TRILHA FORMAL** e o log de passos;
  pressione `Tab` para mostrar ou ocultar essa trilha.
- **Esperado:** ao escolher `S -> aS` a pilha mostra `a` no topo empilhado à
  esquerda, com `S` disponível para nova expansão; ao finalmente escolher
  `S -> ab`, a pilha esvazia e a porta de saída (★) aparece.
- Para o traço formal completo do algoritmo do PDF: em **PREPARAÇÃO LIVRE**,
  clique **GERAR SENTENÇA** repetidamente até obter a sequência de escolhas
  1/1/2 (ou rode `cargo test -p grammar_engine derivation` e veja o teste
  `records_a_full_event_trace_matching_the_pdf_fixture`, que reproduz exatamente
  o fixture do PDF: escolhas 1/1/2, pops `a/a/a/b`, saída `aaab`).

### 4. Apenas gramáticas regulares

- Em **CRIAR GRAMÁTICA**, informe uma gramática livre de contexto, como
  `S -> AB\nA -> a\nB -> b`, e retorne à preparação.
- **Esperado:** o botão **▶ JOGAR LABIRINTO** não é disponibilizado, pois a
  validação aceita somente gramáticas regulares à direita.
- Teste adicional: informe `S -> aB\nB -> aB` (regular, mas improdutiva).
  **Esperado:** a tentativa de iniciar o labirinto falha e a sessão não é
  aberta — ver o teste `start_free_maze_fails_on_an_unproductive_grammar`.

### 5. Interface gráfica com entrada e resultado da derivação

- Toda a interação acima (campo de produções, resultado `G={N,T,P,S}`, botão
  de geração, labirinto jogável, trilha com pilha/passos/regex) é uma
  única aplicação gráfica (`egui` sobre `macroquad`), sem etapas de linha de
  comando após `cargo run -p grammar_quest`.
- Controles: `WASD`/setas movem o personagem, `Tab` mostra/esconde a trilha
  formal e `Esc` volta à preparação.

### 6. Três gramáticas de exemplo no software

- Em **PREPARAÇÃO LIVRE**, clique **ESCOLHER EXEMPLO**.
- **Esperado:** exatamente três opções — **Trilha Inicial**, **Sequência
  a-b-c** e **Encruzilhada Regular** — cada uma com uma descrição e sua
  gramática correspondente (`crates/grammar_engine/src/examples.rs`,
  `EXAMPLE_SOURCES`). Selecionar qualquer uma ativa a gramática escolhida; as
  três são jogáveis no labirinto.

### 7. Conversão da gramática regular para expressão regular após a derivação

- Com **Trilha Inicial** ativa, clique **GERAR SENTENÇA** (ou jogue o
  labirinto até vencer).
- No resultado formal, observe a expressão regular e expanda a seção
  **"Equações e eliminação de variáveis"**
  (`crates/grammar_quest/src/ui/side_panel.rs`).
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

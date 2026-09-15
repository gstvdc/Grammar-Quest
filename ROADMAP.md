# Roadmap — Grammar Quest (Grammar Maze)

Cada fase entrega algo rodável. As fases 0+1 sozinhas já cumprem 100% da
especificação do trabalho — servem de rede de segurança caso o tempo
aperte antes da Fase 2/3.

## Fase 0 — Motor de gramática (`grammar_engine`)

- [x] Modelar `Grammar { non_terminals, terminals, productions, start }`.
- [x] Parser de uma notação textual simples (`S -> aS | ab`).
- [x] Validação: gramática é regular (produções só do tipo `A -> aB`,
      `A -> a` ou `A -> ε`).
- [x] Derivação aleatória via **pilha** (`Vec<Symbol>`), seguindo o
      algoritmo do PDF passo a passo, registrando cada produção aplicada.
- [x] Conversão para expressão regular por eliminação de variáveis
      não-terminais (equações à la exemplo do PDF: `S = aS + ab` → `a*ab`).
- [x] Três gramáticas de exemplo embutidas.
- [x] Testes unitários cobrindo o exemplo exato do PDF (`S ::= aS | ab` →
      regex `a*ab`).

## Fase 1 — Casca do app + painel (`grammar_quest`)

- [x] Janela macroquad + overlay egui-macroquad.
- [x] Formulário de entrada da gramática (N, T, P, S) ou seletor das 3
      gramáticas prontas.
- [x] Botão "gerar sentença aleatória" chamando `grammar_engine`.
- [x] Painel lateral: pilha (estado atual), log de derivação passo a
      passo, sentença final, expressão regular.
- [x] Neste ponto: **especificação do professor 100% atendida.**

## Fase 2 — Grammar Maze

- [x] Tela de labirinto reaproveitando o painel lateral da Fase 1.
- [x] Bifurcações do labirinto = produções do não-terminal atual;
      escolher um corredor = aplicar aquela produção (empilha/desempilha
      de verdade, ao vivo, refletido no painel).
- [x] Chegar a uma produção só-terminal fecha a pilha e libera a saída.
- [x] Pelo menos as 3 gramáticas de exemplo devem ser jogáveis como
      labirinto.

## Fase 3 — Polish ("fluido e bonito")

- [x] Animação de push/pop da pilha (tween, não corte seco).
- [x] Log de derivação com efeito de digitação.
- [x] Tema visual coeso (paleta, tipografia, transições entre telas).
- [x] Portas-puzzle: sentenças falsas geradas e validadas contra a regex
      derivada (garantia de que as opções erradas realmente não pertencem
      à linguagem).
- [x] Tela de vitória / pontuação.

Validação manual da janela pendente neste ambiente: `cargo run -p grammar_quest`
aborta durante a inicialização do menu do macOS, dentro de `miniquad`, porque
`NSRunningApplication.localizedName` retorna nulo. `cargo test`, Clippy e build
do workspace passam; a conferência visual deve ser repetida em uma execução
macOS com identidade de aplicativo disponível.

## Status honesto (2026-09-15)

Os itens `[x]` acima descrevem funcionalidade que existe e funciona (o motor,
a derivação via pilha, o labirinto, os três presets, a conversão para regex).
"Concluído" aqui quer dizer *núcleo funcional pronto*; o polimento acadêmico
foi rastreado ao longo do dia em três rodadas sucessivas de auditoria e ajuste
(`docs/audits/2026-09-15-project-audit.md`, `docs/tasks/2026-09-15-audit-adjustments.md`),
cada tarefa só marcada feita depois de verificada manualmente contra seu
critério de aceite. Este é o estado final, consolidado:

**Concluído e verificado:**

- **T1 — ficha `G={N,T,P,S}`:** `Grammar::overview()` +
  `show_grammar_overview_card` mostram `N`, `T`, `S` e `P` por extenso, com
  explicação de que `S` é o primeiro lado esquerdo declarado; testes provam
  `S -> aS | ab` → `N={S}`, `T={a,b}`, `S=S`, 2 produções, e que entrada
  inválida limpa o preview.
- **T2 — sessão validada antes do labirinto** (escopo reduzido, aprovado
  pelo usuário — não a arquitetura `GrammarSession` originalmente cogitada):
  `start_free_maze` agora exige uma derivação bem-sucedida (`derive_random`)
  e uma conversão para regex bem-sucedida (`to_regex_trace`) antes de mudar
  para `ScreenMode::Playing`; `start_difficulty_maze` já tinha essa garantia
  via `derive_random_in_step_range`, e ganhou a checagem de regex. `begin_maze`
  virou um commit puro — só muta `AppState` depois de ambas as validações.
  Teste: `start_free_maze_fails_on_an_unproductive_grammar` prova que
  `"S -> aB\nB -> aB"` (regular, improdutiva) não entra mais em `Playing`.
- **T3 — traço completo de derivação e regex:** o teste
  `records_a_full_event_trace_matching_the_pdf_fixture` reproduz o fixture
  exato do PDF (escolhas 1/1/2, pops `a/a/a/b`, saída `aaab`) e o painel
  lateral mostra equações/eliminações antes da regex final, não só o
  resultado.
- **T4 — `DoorKind` e cooldown:** `Door.is_exit: bool` virou
  `Door.kind: DoorKind` (variantes `Production`/`Exit`, ambas testadas); uma
  colisão com porta errada arma um cooldown de 0.6s
  (`AppState::door_cooldown`) que suprime novas tentativas enquanto o
  jogador ainda está sobre a porta, evitando repetir a penalidade a cada
  frame. Assets sem uso (`floor_metal.png`, `wall_glass.png`,
  `tiles/LICENSE-CC0.txt`) já haviam sido removidos numa rodada anterior.
- **T5 — `main.rs` dividido:** 729 → 32 linhas, extraído em
  `app.rs`/`gameplay.rs`/`effects.rs`/`render/{mod,world,portals}.rs`.
- **T8 — checklist de aceite:** `docs/acceptance-checklist.md` liga cada um
  dos sete requisitos a uma ação na UI e ao resultado esperado; README fixa a
  versão de Rust verificada (`rustc 1.98.0`) e aponta a nota do panic
  conhecido do `miniquad` no macOS para o checklist.

**Deliberadamente não iniciado (fora de escopo por decisão do usuário, não
por falta de tempo):**

- **T6 — produtividade/equivalência formal por análise de ponto fixo:**
  `derive_random` continua usando o limite de 10.000 passos
  (`MAX_DERIVATION_STEPS`) como sinal de improdutividade em vez de um cálculo
  dedicado de não-terminais produtivos; não há `analysis.rs` nem comparação
  de equivalência exaustiva regex↔gramática. T2 reaproveita esse mesmo sinal
  de falha em vez de depender de T6, então a lacuna prática do T2 original
  (gramática improdutiva jogável) já está fechada sem essa engenharia extra.
- **T7 — responsividade/tokens/acessibilidade:** não existem
  `ui/tokens.rs`/`viewport.rs`; o layout permanece fixo em `1280.0`/`800.0`
  em `app.rs`/`main.rs`/`maze.rs`, sem leitura de
  `screen_width()/screen_height()`. Avaliado contra o escopo YAGNI deste
  trabalho acadêmico (avaliação por um único professor, não múltiplos
  dispositivos) e rejeitado como desproporcional.

Achado incidental (rodada de T2): `crates/grammar_quest/src/audio.rs` não
compilava contra o `macroquad` fixado (`play_sound` exige `&Sound`), mascarado
por cache de build incremental — corrigido como ajuste cirúrgico por bloquear
qualquer build limpo, algo relevante para o T8 acima.

## Fora de escopo (YAGNI por enquanto)

- Gramáticas livres de contexto ou sensíveis ao contexto.
- Multiplayer, save/load, sons.
- Qualquer ferramenta de indexação/grafo de código externa.

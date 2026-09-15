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
Eles **não** significam que não há mais nada a fazer: `docs/audits/2026-09-15-project-audit.md`
documenta, no mesmo dia, lacunas reais de acabamento acadêmico contra o mesmo
código. "Concluído" aqui quer dizer *núcleo funcional pronto*; os gaps de
polimento acadêmico seguem rastreados em `docs/tasks/2026-09-15-audit-adjustments.md`
e não estão marcados como feitos até serem verificados manualmente:

- **T1 e T3 (verificados e concluídos em 2026-09-15):** checados contra o
  critério de aceite completo, não só lidos. T1: `Grammar::overview()` +
  `show_grammar_overview_card` mostram `N`, `T`, `S` e `P` por extenso na
  ficha `G = {N, T, P, S}`, com explicação de que `S` é o primeiro lado
  esquerdo; testes existentes provam `S -> aS | ab` → `N={S}`, `T={a,b}`,
  `S=S`, 2 produções, e que entrada inválida limpa o preview. T3: o teste
  `records_a_full_event_trace_matching_the_pdf_fixture` reproduz o fixture
  exato do PDF (escolhas 1/1/2, pops `a/a/a/b`, saída `aaab`) e o painel
  lateral mostra equações/eliminações antes da regex final, não só o
  resultado. Ver `docs/tasks/2026-09-15-audit-adjustments.md` para o
  detalhamento.
- **P0 - Sessão atômica antes do labirinto** (T2, ainda parcial): `start_free_maze` valida
  parsing e regularidade, mas não produtividade; `begin_maze` calcula o traço
  de regex com `.ok()`, então uma gramática que entra em `Playing` pode não
  ter regex derivada sem que isso apareça como erro para o jogador. Confirmado
  nesta rodada com um teste descartável: `"S -> aB\nB -> aB"` (improdutiva)
  entra em `Playing` normalmente.
- **P1 - Garantias de produtividade/geração** (T6, ainda não iniciado): `derive_random` ainda
  depende de um limite de passos (10.000) em vez de um cálculo de
  não-terminais produtivos; uma gramática regular mas improdutiva (`S -> aS`)
  gasta as 10.000 iterações completas (~55ms, confirmado nesta rodada) antes
  de falhar, em vez de detectar a improdutividade de imediato. Não há
  `analysis.rs` nem comparação de equivalência exaustiva.
- **P1 UX - Responsividade** (T7, não iniciado): confirmado que não existe
  `ui/tokens.rs`/`viewport.rs` e que o layout permanece fixo em
  `1280.0`/`800.0` em `app.rs`, `main.rs` e `maze.rs`, sem leitura de
  `screen_width()/screen_height()`.
- **P1 entrega - Checklist** (T8, não iniciado): confirmado que
  `docs/acceptance-checklist.md` não existe e o README não fixa versão
  mínima de Rust/macOS nem tem roteiro requisito-por-requisito.

Concluído nesta rodada de limpeza (ver `docs/tasks/2026-09-15-audit-adjustments.md`):
remoção dos assets de tile sem uso (T4, parte de assets), divisão de
`main.rs` em `app.rs`/`gameplay.rs`/`effects.rs`/`render/` (T5), e a
verificação completa de T1/T3 (concluídas). O restante de T4 (troca de
booleanos por `DoorKind`, cooldown pós-erro) e as tasks T2/T6/T7/T8 seguem
pendentes, cada uma como um esforço de engenharia dedicado, não uma limpeza
pontual. Um achado extra desta rodada: `crates/grammar_quest/src/audio.rs`
não compilava contra o `macroquad` fixado (assinatura `&Sound`), mascarado
por cache de build incremental — corrigido como ajuste cirúrgico por
bloquear qualquer build limpo.

## Fora de escopo (YAGNI por enquanto)

- Gramáticas livres de contexto ou sensíveis ao contexto.
- Multiplayer, save/load, sons.
- Qualquer ferramenta de indexação/grafo de código externa.

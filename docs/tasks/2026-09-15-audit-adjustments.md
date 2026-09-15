# Tasks de ajustes pós-auditoria

Base: `docs/audits/2026-09-15-project-audit.md`

Cada task foi delimitada para um subagente, com branch/worktree e commit
isolados. Tasks que alteram os mesmos arquivos devem ser sequenciais.

## Dependências

```text
T1 Conformidade G ──┐
T2 Sessão atômica ──┼──> T5 Refatorar main ──> T7 UI responsiva
T3 Traço formal ────┘             │
T4 Final + assets ────────────────┘

T6 começa após T2. T8 começa após T1, T3, T4 e T7.
```

T1 e T3 podem rodar em paralelo se T1 não editar `side_panel.rs`. T2 e T4
editam estado/maze e devem ser sequenciais. T5 começa após integrar T1-T4.

## T1 - Expor `G={N,T,P,S}`

**Prioridade:** P0. **Objetivo:** demonstrar o primeiro requisito sem duplicar
o parser.

**Arquivos:** `grammar_engine/src/grammar.rs`, `grammar_quest/src/state.rs`,
`grammar_quest/src/ui/editor.rs` e testes locais.

**Escopo:** expor getters/projeção ordenada dos componentes; fazer preview ao
editar; renderizar ficha `N/T/P/S`; explicar que `S` é o primeiro LHS; renomear
o botão para “Gerar sentença aleatória”; mostrar descrição do preset.

**Aceite:** `S -> aS | ab` mostra `N={S}`, `T={a,b}`, `S=S` e duas produções;
entrada inválida limpa preview; UI não interpreta símbolos; três presets seguem
selecionáveis.

```bash
cargo test -p grammar_engine
cargo test -p grammar_quest
cargo clippy --workspace --all-targets -- -D warnings
```

**Status:** concluído, verificado em 2026-09-15. `Grammar::overview()`
(`crates/grammar_engine/src/grammar.rs:16-73`) expõe `GrammarOverview` com
`non_terminals`/`terminals`/`start`/`production_lines`/`production_count`, e
`show_grammar_overview_card` em `crates/grammar_quest/src/ui/editor.rs:225-283`
renderiza a ficha `G = {N, T, P, S}` lendo apenas `AppState::grammar_preview`
(sem reinterpretar a gramática). Confirmado via os testes já existentes de
`state.rs`: `starts_with_a_grammar_preview_for_the_default_example` (linha
440) prova que `S -> aS | ab` produz exatamente `N={S}`, `T={a,b}`, `S=S` e
`production_count=2`; `invalid_grammar_text_clears_the_preview` (linha 459)
prova que `S aS` deixa `grammar_preview` como `None`;
`valid_edits_refresh_the_preview_live` (linha 466) prova atualização ao
digitar. O botão em `editor.rs:200-211` tem o texto literal "Gerar sentença
aleatória" (com prefixo de ícone ⚡, que não altera o texto exigido); os três
presets seguem selecionáveis via `EXAMPLE_SOURCES` e cada um mostra sua
`description` em itálico abaixo do combo (`editor.rs:59-67`). Todos os
`cargo test -p grammar_engine`/`-p grammar_quest`/`clippy --workspace
--all-targets` listados acima passam sem avisos novos. Nenhum ajuste foi
necessário.

## T2 - Criar sessão validada e estado transacional

**Prioridade:** P0. **Objetivo:** impedir sessões que falham tarde e mutações
parciais.

**Arquivos:** `grammar_quest/src/state.rs`, novo `session.rs` se útil e pontos
de consumo em `main.rs`.

**Escopo:** `GrammarSession` com gramática, regex e derivação; `start_maze`
valida parsing, regularidade, produtividade e regex antes de mudar o modo;
puzzle é preparado antes do commit; remover `unwrap_or_default()` de regex;
falhas preservam estado anterior.

**Aceite:** improdutiva não entra em `Playing`; erro de regex é visível;
`to_regex` não roda por frame; teste compara estado observável antes/depois de
uma falha.

```bash
cargo test -p grammar_quest state
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

**Status:** parcial, verificado em 2026-09-15. Feito: o antigo
`unwrap_or_default()` sobre regex citado no achado original não existe mais
(`rg 'unwrap_or_default' crates/` só encontra um uso não relacionado, um
`Vec<Symbol>` vazio de fallback em `state.rs:317`); `start_free_maze`
(`state.rs:223-246`) valida parsing e regularidade antes de mudar de modo, e
uma falha preserva o estado anterior — confirmado pelo teste existente
`start_free_maze_fails_on_invalid_grammar` (`state.rs:540`), que checa
`state.mode` continua `Laboratory` e `error_message` é populado. Faltando:
`start_free_maze`/`start_difficulty_maze` **não** validam produtividade nem
pré-computam com sucesso a conversão para regex antes do commit —
`begin_maze` (`state.rs:278-289`) chama
`to_regex_trace(&grammar).ok()`, descartando silenciosamente qualquer erro de
regex em vez de bloquear a transição. Verificado com um teste descartável
(removido após a checagem, não commitado): uma gramática regular mas
improdutiva `"S -> aB\nB -> aB"` passa por `start_free_maze()` com
`Ok(())` e `state.mode == ScreenMode::Playing`, violando o critério de aceite
"improdutiva não entra em `Playing`". `to_regex` não é recomputado por frame
(só em `generate()`, `begin_maze()` e na conclusão do labirinto), então essa
parte do aceite já está ok. Não implementado como "correção pequena" porque
fechar a lacuna exige a arquitetura `GrammarSession`/commit atômico descrita
no escopo (checar produtividade + regex antes de mutar `AppState`, com
reversão real em caso de erro) — isso é engenharia de feature, não um ajuste
cirúrgico, e fica para uma sessão dedicada.

## T3 - Traço completo da pilha e da regex

**Prioridade:** P0 acadêmico. **Objetivo:** reproduzir as páginas 4 e 5 do PDF.

**Arquivos:** `grammar_engine/src/derivation.rs`, `regex_conversion.rs`,
`lib.rs`, `grammar_quest/src/ui/side_panel.rs` e `game_hud.rs`.

**Escopo:** eventos `ProductionChosen`, `Pushed`, `TerminalPopped`, `Completed`;
resultado de regex com equações iniciais, eliminações e expressão final; seções
recolhíveis; animação baseada nos eventos reais.

**Aceite:** o fixture do PDF exibe `S=aS+ab`, escolhas 1/1/2, pops `a/a/a/b`,
saída `aaab`, pilha vazia e `a*ab`; motor não depende da UI; snapshots antes e
depois são inequívocos.

```bash
cargo test -p grammar_engine derivation
cargo test -p grammar_engine regex_conversion
cargo test --workspace
```

**Status:** concluído, verificado em 2026-09-15. `DerivationEvent` em
`crates/grammar_engine/src/derivation.rs:23-40` define exatamente
`ProductionChosen`/`Pushed`/`TerminalPopped`/`Completed`, e o teste
`records_a_full_event_trace_matching_the_pdf_fixture`
(`derivation.rs:336-386`) reproduz o fixture do PDF ponta a ponta: gramática
`S -> aS | ab`, escolhas `[0, 0, 1]` (equivalente às escolhas 1/1/2 do slide,
0-indexadas), pops `['a','a','a','b']`, saída final `"aaab"`, e o último
evento é `Completed { sentence: "aaab" }`. `to_regex_trace`
(`regex_conversion.rs`) devolve `RegexTrace{ initial_equations,
eliminations, final_expression }`; o teste
`traces_the_pdf_fixture_equation_and_elimination` e
`matches_the_exact_pdf_example_output` confirmam `final_expression == "a*ab"`
e a equação resolvida `"S=a*ab"`. O motor não depende de UI (todo o rastro
vive em `grammar_engine`, sem `use egui`/`use macroquad`). No lado da UI,
`crates/grammar_quest/src/ui/side_panel.rs:135-180` (`show_regex_trace`)
usa `egui::CollapsingHeader` para mostrar as equações iniciais e as
eliminações antes da expressão final — não apenas o resultado. `cargo test -p
grammar_engine derivation` (8 testes) e `regex_conversion` (7 testes) e
`cargo test --workspace` passam. Nenhum ajuste foi necessário.

## T4 - Unificar final e remover assets mortos

**Prioridade:** P1. **Objetivo:** eliminar fluxos concorrentes e arquivos sem
uso.

**Arquivos:** `state.rs`, `maze.rs`, `main.rs`, `assets/ATTRIBUTION.md` e tiles.

**Escopo:** adotar derivação -> puzzle -> portal -> vitória; trocar booleanos
por `DoorKind`; aplicar cooldown após erro; remover `floor_metal.png`,
`wall_glass.png` e licença CC0 se ainda sem referências.

**Status:** parcialmente concluído em 2026-09-15 — apenas a remoção dos
assets sem uso. `floor_metal.png`, `wall_glass.png` e `tiles/LICENSE-CC0.txt`
foram removidos e `ATTRIBUTION.md` atualizado (confirmado via `rg
'floor_metal|wall_glass' crates/grammar_quest` sem resultados antes e depois).
A troca de `DoorKind`, o cooldown pós-erro e a unificação do fluxo de final
**não** foram feitos — seguem pendentes para uma sessão dedicada de
gameplay.

**Aceite:** todo `DoorKind` é alcançável e testado; vitória ocorre no portal;
colisão contínua não repete erros; `rg 'floor_metal|wall_glass'
crates/grammar_quest` não encontra uso.

```bash
cargo test -p grammar_quest maze
cargo test -p grammar_quest state
cargo test --workspace
git diff --check
```

## T5 - Dividir runtime e remover duplicação por frame

**Prioridade:** P1. **Objetivo:** deixar `main.rs` como composição de alto nível.

**Arquivos novos:** `app.rs`, `gameplay.rs`, `effects.rs`, `render/mod.rs`,
`render/world.rs`, `render/portals.rs`. Modificar `main.rs` e `state.rs`.

**Escopo:** mover efeitos, desenho e gameplay; fornecer projeção emprestada ao
painel; impedir conversão de regex e clone do histórico no render.

**Aceite:** `main.rs` abaixo de 180 linhas; render não altera `AppState`;
gameplay testável sem janela; nenhuma conversão/clone do log por frame.

```bash
cargo test -p grammar_quest
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
```

**Status:** concluído em 2026-09-15. `main.rs` (729 linhas) foi dividido em
`app.rs` (loop macroquad, transições de tela, composição egui do laboratório
e do HUD), `gameplay.rs` (movimento do jogador, poeira de passos, colisão de
porta e aplicação da escolha), `effects.rs` (partículas, ondas de choque e
texto flutuante, agrupados em `EffectsState`) e `render/{mod,world,portals}.rs`
(arena/grade/sigilo/paredes e portas/placas/beacons). `main.rs` ficou com 32
linhas (abaixo do alvo de 180). O render não muta `AppState` (recebe
referências emprestadas); `gameplay::update_playing` só toca `AppState` via
`apply_maze_choice`, como antes. `cargo build --workspace`, `cargo clippy
--workspace --all-targets` (0 avisos), `cargo test -p grammar_quest` (22/22) e
`cargo test -p grammar_engine` (47/47) passam. Comportamento do jogo não foi
alterado, exceto por uma correção de ordem-z incidental à extração: efeitos
de partícula agora desenham explicitamente antes do sprite do jogador e o
texto flutuante depois, preservando a ordem visual original que dependia da
ordem de inserção no loop de `main.rs`.

## T6 - Produtividade e equivalência formal

**Prioridade:** P1. **Objetivo:** ampliar garantias além das amostras geradas.

**Arquivos:** novo `grammar_engine/src/analysis.rs`, `derivation.rs`, `lib.rs` e
testes de `regex_conversion.rs`.

**Escopo:** ponto fixo de não-terminais produtivos; erro específico para
linguagem vazia; geração com orçamento e rota terminal; enumerar todas as
palavras até tamanho 5 e comparar aceitação nos dois sentidos para fixtures com
epsilon, união, recursão, cadeia e ciclos.

**Aceite:** `S -> aS` falha antes de 10.000 passos; gramáticas produtivas
terminam no orçamento; comparação exaustiva limitada passa; o PDF continua
retornando literalmente `a*ab`.

```bash
cargo test -p grammar_engine
cargo clippy -p grammar_engine --all-targets -- -D warnings
```

**Status:** não iniciado, verificado em 2026-09-15. Não existe
`grammar_engine/src/analysis.rs` nem qualquer cálculo de ponto fixo de
não-terminais produtivos (`rg 'productiv' crates/grammar_engine/src/*.rs` só
encontra a palavra em um comentário de `grammar.rs:149` e em
`lib.rs:34`, nenhum dos dois é uma análise de produtividade real). A
detecção de gramática improdutiva continua sendo o limite fixo
`MAX_DERIVATION_STEPS = 10_000` (`derivation.rs:9`) somado a
`GrammarError::DerivationTooLong`. Verificado com um teste descartável
(removido após a checagem): `derive_random` sobre `"S -> aS"` (improdutiva)
gasta de fato as 10.000 iterações antes de falhar (~55ms), em vez de
detectar `S` como não-produtivo de imediato — viola o aceite "`S -> aS`
falha antes de 10.000 passos" tomado literalmente (hoje ela falha *depois*
de esgotar exatamente esse orçamento, não antes dele). Não há também
nenhuma comparação de equivalência exaustiva entre a regex derivada e a
gramática (nenhum teste enumera palavras até tamanho 5 nos dois sentidos);
os testes de regex existentes (`generated_sentences_always_match_the_derived_regex`
etc.) validam amostras geradas, não uma enumeração exaustiva limitada. Não
implementado como ajuste pequeno: exige uma análise de ponto fixo nova, um
tipo de erro específico para linguagem vazia e um gerador exaustivo por
tamanho — engenharia de feature real, não um ajuste cirúrgico. Fica para uma
sessão dedicada.

## T7 - Responsividade, tokens e acessibilidade

**Prioridade:** P1 UX. **Objetivo:** preservar o Neon Arcade com leitura clara.

**Arquivos:** novos `ui/tokens.rs` e `viewport.rs`; módulos `ui/`, `render/` e,
se necessário, `player.rs`.

**Escopo:** centralizar cores/métricas/durações; usar viewport real e reservar
HUD/painel; HUD quebra em duas linhas; trilha vira overlay em largura pequena;
portas clicáveis; onboarding; feedback além de cor; reduzir animações; revisar
emojis e foco.

**Aceite:** 800x600, 1280x800 e 1920x1080 sem sobreposição; fluxo por teclado e
mouse; texto normal com contraste WCAG AA; RGB do produto apenas em tokens.

```bash
cargo test -p grammar_quest
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p grammar_quest
```

Anexar capturas das três resoluções e do fluxo laboratório -> derivação ->
puzzle -> saída -> vitória.

**Status:** não iniciado, verificado em 2026-09-15. Não existem
`crates/grammar_quest/src/ui/tokens.rs` nem `viewport.rs` (`find
crates/grammar_quest/src -iname "*token*" -o -iname "*viewport*"` não retorna
nada). O layout permanece com dimensões fixas espalhadas pelo código:
`crates/grammar_quest/src/app.rs:12-13` define
`VIEWPORT_W/H = 1280.0/800.0` como constantes, `main.rs:19` fixa
`window_width: 1280` na configuração da janela do macroquad, e
`crates/grammar_quest/src/maze.rs` chama `Room::build(..., 1280.0, 800.0)`
em quatro pontos (mais um caso com `800.0, 600.0` só em teste). Nenhum desses
pontos lê `macroquad::window::screen_width()/screen_height()`. Portanto não
há reserva dinâmica de HUD/painel, nem quebra de HUD em duas linhas, nem
overlay para a trilha em janelas pequenas — todo o pedido de T7 (viewport
real, tokens centralizados, portas clicáveis, onboarding, contraste,
revisão de emojis) segue pendente. Não atacado como ajuste pequeno: é
exatamente o tipo de rework arquitetural de UI que a tarefa deste round
proíbe tentar como correção cirúrgica. Fica para uma sessão dedicada.

## T8 - Checklist de entrega e smoke test macOS

**Prioridade:** P1 entrega. **Objetivo:** avaliação reproduzível em outra
máquina.

**Arquivos:** `README.md`, novo `docs/acceptance-checklist.md` e `ROADMAP.md`
somente mediante evidência.

**Escopo:** investigar o panic do `miniquad` sem bundle; documentar execução
macOS suportada e Rust mínimo; roteiro para cada requisito e três presets;
resultado `aaab`/`a*ab`; registrar limitações reais.

**Aceite:** máquina limpa executa pelo README; checklist liga requisitos a
ações visíveis; roadmap só marca itens manualmente observados.

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release --workspace
```

**Status:** não iniciado, verificado em 2026-09-15. `docs/acceptance-checklist.md`
não existe. O `README.md` documenta um pré-requisito genérico ("stable Rust
toolchain with Rust 2024 edition support", linha 53) e traz uma nota sobre o
panic conhecido do `miniquad` em automação macOS sem identidade de app
(seção "macOS note"), mas não fixa uma versão mínima de Rust nem de macOS, e
não existe um roteiro que amarre cada um dos sete requisitos do PDF (e os
três presets) a uma ação visível na UI com o resultado esperado
(`aaab`/`a*ab`). Isso confirma o achado original do audit — nada mudou desde
então. Não atacado como ajuste pequeno: escrever esse checklist é trabalho de
autoria de conteúdo (não uma correção de código) e deve ser feito por quem
vai de fato rodar o roteiro descrito, então fica para uma sessão dedicada.

## Achados adicionais desta rodada de verificação (2026-09-15)

Durante a verificação de T2, um `cargo build -p grammar_quest` "do zero"
(fora do cache de incremental compilation que escondia o problema) revelou
que `crates/grammar_quest/src/audio.rs` não compilava contra o `macroquad
0.4.16` fixado em `Cargo.lock`: `audio::play_sound` exige `&Sound`, mas
`play_once` e as quatro chamadas em `Sfx::play_*` passavam `Sound` por
valor. Isso não é uma regressão desta sessão — o arquivo não estava listado
como modificado no `git status` inicial e não faz parte do escopo de
T1/T2/T3/T6/T7/T8 — mas bloqueava qualquer build limpo (por exemplo em uma
outra máquina, ou depois de `cargo clean`), o que é diretamente relevante
para T8 ("máquina limpa executa pelo README"). Foi corrigido como ajuste
cirúrgico (assinatura de `play_once` passou a receber `&Sound`, chamadas
passaram a emprestar com `&self.campo`) porque sem isso nenhuma verificação
subsequente de build/test/clippy seria confiável. `cargo build --workspace`,
`cargo clippy --workspace --all-targets` (0 avisos) e as suítes de teste de
`grammar_engine` (47/47) e `grammar_quest` (22/22) voltaram a passar depois
do ajuste.


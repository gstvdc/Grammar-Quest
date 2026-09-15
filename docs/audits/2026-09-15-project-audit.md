# Auditoria do projeto Grammar Quest

Data: 2026-09-15

## Atualização (2026-09-15, pós-auditoria — estado final)

Este documento é um retrato do projeto no início do dia acima; os achados
originais na seção "Correção e robustez" abaixo não foram reescritos, só
resolvidos ao longo de várias rodadas de verificação e ajuste no mesmo dia.
Esta seção resume o estado final consolidado — ver
`docs/tasks/2026-09-15-audit-adjustments.md` para o histórico completo
arquivo-por-arquivo de cada rodada.

**Concluído e verificado contra critério de aceite (não só lido):**

- **T1 — ficha `G={N,T,P,S}`:** `Grammar::overview()` e
  `show_grammar_overview_card` (`crates/grammar_quest/src/ui/editor.rs`)
  cobrem o pedido; testes provam `S -> aS | ab` → `N={S}`, `T={a,b}`, `S=S`,
  2 produções, entrada inválida limpa o preview, os três presets continuam
  selecionáveis com descrição.
- **T2 — sessão validada antes do labirinto** (escopo reduzido e aprovado
  pelo usuário, não a arquitetura `GrammarSession` completa cogitada
  abaixo): `start_free_maze` agora exige `derive_random` e `to_regex_trace`
  bem-sucedidos antes de mudar para `Playing`; `start_difficulty_maze` já
  tinha a garantia de produtividade via `derive_random_in_step_range` e
  ganhou a checagem de regex. Teste
  `start_free_maze_fails_on_an_unproductive_grammar` prova que
  `"S -> aB\nB -> aB"` (regular, improdutiva) não entra mais em `Playing`.
- **T3 — traço de derivação e regex:** `records_a_full_event_trace_matching_the_pdf_fixture`
  reproduz o fixture exato do PDF; o painel lateral mostra equações e
  eliminações antes da regex final.
- **T4 — `DoorKind` e cooldown:** `Door.is_exit: bool` virou
  `Door.kind: DoorKind` (`Production`/`Exit`, ambas testadas); uma colisão
  com porta errada arma um cooldown de 0.6s que evita repetir a penalidade a
  cada frame enquanto o jogador ainda toca a porta. Os assets sem uso
  (`floor_metal.png`, `wall_glass.png`, `tiles/LICENSE-CC0.txt`) já tinham
  sido removidos numa rodada anterior.
- **T5 — `main.rs` dividido:** 729 → 32 linhas, extraído em
  `app.rs`/`gameplay.rs`/`effects.rs`/`render/{mod,world,portals}.rs`.
- **T8 — checklist de aceite:** `docs/acceptance-checklist.md` liga cada
  requisito a uma ação e resultado esperado na UI; README fixa a versão de
  Rust verificada e referencia o checklist para o smoke test macOS.

**Deliberadamente não iniciado (rejeitado por desproporcional ao escopo
desta disciplina, decisão do usuário — não uma lacuna esquecida):**

- **T6 — ponto fixo de produtividade e equivalência exaustiva:**
  `derive_random` continua usando o limite fixo de 10.000 passos como sinal
  de improdutividade; T2 reaproveita esse mesmo sinal em vez de depender de
  uma análise dedicada, então a lacuna prática (gramática improdutiva
  jogável) já está fechada sem essa engenharia extra.
- **T7 — responsividade/tokens/acessibilidade:** layout permanece fixo em
  `1280.0`/`800.0`; nenhum `ui/tokens.rs`/`viewport.rs` foi criado.

Achado incidental de uma rodada intermediária: `crates/grammar_quest/src/audio.rs`
não compilava contra o `macroquad 0.4.16` fixado (`play_sound` exige
`&Sound`, o código passava `Sound` por valor) — mascarado por cache de
compilação incremental. Corrigido como ajuste cirúrgico por bloquear
qualquer build limpo, algo diretamente relevante para T8.

## Escopo e evidências

Esta auditoria compara todo o projeto com as oito páginas de
`context/td01-linguagens-formais.pdf`. Foram revisados o código Rust,
assets, documentos e testes. O PDF foi extraído e também renderizado para
conferência visual das páginas de requisitos, algoritmo, pilha, derivação,
eliminação de variáveis e entrega.

Verificações executadas:

```bash
pdfinfo "context/td01-linguagens-formais.pdf"
pdftotext -layout "context/td01-linguagens-formais.pdf" -
pdftoppm -png -r 120 "context/td01-linguagens-formais.pdf" /tmp/grammar-quest-pdf/page
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Resultado: 39 testes do `grammar_engine` e 17 do `grammar_quest` passam;
Clippy termina sem avisos. O smoke test gráfico não foi possível neste ambiente:
`miniquad 0.4.11` aborta ao iniciar o menu do macOS quando
`NSRunningApplication.localizedName` retorna nulo.

## Conformidade com o PDF

| Exigência | Evidência | Situação | Melhoria |
|---|---|---|---|
| Entrada de `G={N,T,P,S}` | O editor recebe `P`; o parser infere `N/T` e usa o primeiro lado esquerdo como `S` | Parcial | Exibir os quatro componentes e permitir/explicar a escolha de `S` |
| Sentenças aleatórias | `derive_random` usa `rand`; `AppState::generate` apresenta o resultado | Atende | Tornar produtividade e limite de geração explícitos |
| Mecanismo de derivação | `DerivationState::apply_choice` expande e consome símbolos | Atende | Registrar cada push e pop para reproduzir o slide |
| Pilha, símbolo esquerdo no topo | `push_production` empilha em ordem inversa; há testes da ordem | Atende | Adicionar teste público da sequência completa do PDF |
| Somente gramáticas regulares | `validate_regular` valida gramáticas lineares à direita | Atende | Documentar se `A -> B` é aceito pela definição da disciplina |
| Interface gráfica de entrada/resultado | egui oferece editor, resultado, pilha, passos e regex | Atende funcionalmente | Validar visualmente, responsividade, teclado e contraste |
| Exemplos no software | Há exatamente três presets no ComboBox | Atende | Mostrar descrição e linguagem de cada preset |
| Regex após derivação | `to_regex` usa eliminação/Arden; caso do PDF retorna `a*ab` | Atende ao exemplo | Exibir equações e eliminações, não apenas a regex final |
| Entrega do código-fonte | Workspace, assets e atribuições estão versionados | Atende | Criar roteiro de apresentação e ambiente reproduzível |

O núcleo cumpre o exemplo executável do professor. A principal lacuna para a
avaliação é demonstrar visualmente `G={N,T,P,S}`. A segunda é pedagógica: o PDF
destaca as equações e a eliminação, enquanto a aplicação mostra apenas a regex
final.

## Correção e robustez

### P0 - Demonstrar `G={N,T,P,S}`

O símbolo inicial é implicitamente a primeira produção e essa regra não aparece
na tela. Recomenda-se manter o editor textual e acrescentar uma ficha “Gramática
interpretada” com conjuntos ordenados `N`, `T`, produções `P` e símbolo `S`,
sempre derivados do `Grammar` do motor.

### P0 - Validar a sessão inteira antes do labirinto

`start_maze` valida parsing e regularidade, mas não produtividade nem conversão
para regex. `apply_maze_choice` altera a pilha antes de criar o puzzle; uma
falha posterior deixa o estado consumido e a sala anterior na tela. Preparar uma
`GrammarSession` completa e fazer commit no estado somente depois que todas as
operações falíveis terminarem.

### P1 - Geração produtiva por construção

`S -> aS` é regular, porém não produz palavra. Hoje `derive_random` tenta até
10.000 passos. Mesmo uma gramática produtiva recursiva tem chance pequena de
atingir esse limite por escolhas uniformes. Calcular não-terminais produtivos e
usar orçamento de tamanho com alternativas que garantam término.

### P1 - Erros de regex não podem virar texto vazio

Há `to_regex(...).unwrap_or_default()` em `state.rs` e `main.rs`. Falhas podem
virar regex vazia e esconder a causa. Encaminhar todo erro para `error_message`
e calcular a regex uma vez por sessão.

### P1 - Registrar o instante correto de cada snapshot

`DerivationStep` guarda pilha após o push e saída antes de consumir os terminais
da nova produção. É um instante válido, mas o painel não explica isso. Modelar
eventos `ProductionChosen`, `Pushed`, `TerminalPopped` e `Completed`, ou guardar
snapshots antes/depois, permite reproduzir a pilha do slide sem ambiguidade.

### P2 - Formalizar a notação aceita

O parser aceita `->`/`::=`, uma letra maiúscula por não-terminal, letras
minúsculas por terminal e `&` ou RHS vazio para epsilon. A UI exibe `ε`, mas o
parser rejeita esse caractere. Documentar o contrato e decidir se também deve
aceitar `ε`.

## Arquitetura, duplicação e código morto

### `main.rs` tem responsabilidades demais

São mais de 700 linhas para loop, telas, colisão, ações, partículas, ondas,
texto flutuante, arena, paredes, portas e composição egui. Extrair:

- `app.rs`: ciclo e transições;
- `gameplay.rs`: colisão, escolhas, cooldown e resultado;
- `effects.rs`: partículas, ondas e textos;
- `render/world.rs`: arena, grade, paredes e sigilo;
- `render/portals.rs`: portas, placas e quebra de linha;
- `viewport.rs`: área real disponível após HUD/painel.

### Cores e métricas estão duplicadas

RGB, espaçamentos, raios, fontes e durações aparecem em `main.rs`, `player.rs` e
todos os módulos UI. `theme.rs` cobre só egui. Criar tokens comuns com
conversores para tipos egui/macroquad.

### Resultado e regex são reconstruídos

`PanelResult` é montado em `generate`, `apply_maze_choice`, `complete_maze` e no
render. Quando a trilha está aberta, `to_regex` e clones do histórico acontecem
a cada frame. Uma `GrammarSession { grammar, regex, derivation }` e uma projeção
emprestada eliminam essa duplicação.

### O fluxo antigo de saída está inalcançável

Ao completar a derivação, sempre nasce um puzzle; acertar muda diretamente para
`Won`. A sala sem puzzle, `Door::is_exit` e o ramo `if door.is_exit` não ocorrem
no fluxo normal. Escolher um final: recomenda-se puzzle -> portal desbloqueado
-> vitória. Se a vitória imediata for mantida, remover a saída antiga e testes.

### Assets sem uso

`floor_metal.png` e `wall_glass.png` não são carregados e somam cerca de 308 KB.
Se não voltarem ao design, remover também `tiles/LICENSE-CC0.txt` e atualizar
`ATTRIBUTION.md`.

### Ruído e encapsulamento

Comentários como “Draw particles” e “Left wall” repetem o código. Muitos campos
internos são públicos, facilitando estados inválidos. Preferir comentários de
decisão, getters e slices (`Option<&[Vec<Symbol>]>`) a `Option<&Vec<_>>`.

## UI e UX

### Hierarquia

O Neon Arcade tem identidade, mas informações avaliadas competem com emojis e
efeitos. Prioridade sugerida: gramática interpretada, ação, sentença,
pilha/passos e regex. O botão deve usar o texto literal “Gerar sentença
aleatória”.

### Responsividade

Sala, poeira e reconstruções usam 1280x800 fixos. O painel direito de 310 px
recobre parte da arena. Derivar tudo de `screen_width()/screen_height()` e
reservar a área do painel. Em telas estreitas, usar HUD em duas linhas e trilha
como overlay.

### Acessibilidade e interação

- oferecer clique/toque nas portas além de WASD/setas;
- manter instruções de controles visíveis no primeiro uso;
- comunicar terminal/não-terminal e erro/acerto por texto/forma além de cor;
- verificar WCAG AA e ausência de glifos emoji;
- oferecer redução de movimento;
- revisar foco, tabulação e ativação por teclado;
- mostrar sucesso “gramática válida” e localizar erro por linha.

### Feedback

- resposta errada teleporta imediatamente e pode repetir colisões;
- a fórmula de pontuação não é explicada;
- a animação chamada push/pop só faz fade do snapshot pós-push;
- validar que o typewriter revela somente a ficha recém-adicionada.

### Direção recomendada

Manter o tema, com laboratório acadêmico como estrutura:

1. cabeçalho curto e estado da gramática;
2. preset, sintaxe e editor;
3. ficha `G={N,T,P,S}`;
4. ações “Gerar sentença aleatória” e “Derivar no labirinto”;
5. linha temporal de produção, push, pops e saída;
6. equações e eliminações de regex recolhíveis;
7. arena usando somente a área livre real.

## Testes: forças e lacunas

Pontos fortes: motor isolado, caso exato do PDF, três presets, ordem da pilha,
parsing inválido, geometria básica e Clippy limpo.

Lacunas:

- projeção de `N/T/P/S` não testada;
- produtividade não é validada antes do jogo;
- não há rollback após operação falível;
- equivalência regex é testada só com sentenças geradas pelo próprio motor;
- teste UI apenas confirma que uma função renderiza;
- geometria não considera painel lateral e poucos viewports;
- smoke test macOS está bloqueado pelo panic do backend neste ambiente.

## Ordem recomendada

1. Conformidade acadêmica (`N/T/P/S`, traço da regex e notação).
2. Sessão atômica, produtividade e erros.
3. Final único e remoção de código/assets mortos.
4. Extração de `main.rs` e tokens visuais.
5. Responsividade e acessibilidade.
6. Testes formais e checklist de apresentação.


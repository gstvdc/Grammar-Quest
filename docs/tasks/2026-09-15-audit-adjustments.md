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

## T4 - Unificar final e remover assets mortos

**Prioridade:** P1. **Objetivo:** eliminar fluxos concorrentes e arquivos sem
uso.

**Arquivos:** `state.rs`, `maze.rs`, `main.rs`, `assets/ATTRIBUTION.md` e tiles.

**Escopo:** adotar derivação -> puzzle -> portal -> vitória; trocar booleanos
por `DoorKind`; aplicar cooldown após erro; remover `floor_metal.png`,
`wall_glass.png` e licença CC0 se ainda sem referências.

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


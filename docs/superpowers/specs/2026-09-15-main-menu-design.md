# Design: Tela de Menu Principal (Grammar Quest)

Data: 2026-09-15

## Status de implementação

Implementado em 2026-09-15: menu principal, seleção de modo, tela de
opções com volume não persistente e créditos, laboratório focado no modo
escolhido e a fonte Press Start 2P. As decisões abaixo permanecem como o
registro de design aprovado.

## Contexto e objetivo

Hoje o jogo abre direto na tela `Laboratory` (editor de gramática + painel
lateral), sem nenhuma tela de boas-vindas. O pedido é substituir essa
abertura por um menu principal de verdade — logo/nome, atmosfera, botão
Jogar, Opções, Sair — no estilo de um RPG retrô 16-bit, e reformular o
`Laboratory` para ser focado no modo de jogo escolhido no menu (em vez do
toggle Livre/Enigma que existe hoje).

Este documento não altera nada em `grammar_engine` nem nos 7 requisitos
não-negociáveis de `CLAUDE.md` — a mudança é inteiramente na camada de jogo
(`grammar_quest`), que já é onde o `CLAUDE.md` autoriza mudanças livres desde
que não dupliquem ou quebrem a lógica de gramática.

## Direção visual aprovada

Decidida por brainstorming visual (mockups HTML descartáveis em
`.superpowers/brainstorm/`, não versionados):

- **Estilo:** "RPG Retrô 16-bit" — paleta quente (roxo escuro de fundo,
  dourado de destaque, vermelho no contorno do título), traço grosso,
  sombra deslocada tipo SNES. Deliberadamente diferente da paleta
  synthwave/neon do `DESIGN.md` atual — é uma tela de abertura com
  identidade própria, não uma extensão do tema do Laboratory/labirinto.
- **Atmosfera:** partículas douradas flutuantes ("fagulhas/vagalumes"),
  subindo devagar e desaparecendo, reaproveitando o sistema de partículas
  já existente em `effects.rs`.
- **Fidelidade ao mockup:** o resultado final deve ser fiel ao mockup
  aprovado — sem comprometer para reaproveitar o logo neon atual
  (`grammar-quest-logo.png`) ou o sprite do swordsman. O menu é 100% visual
  novo: nome em fonte pixel, mascote como forma geométrica simples
  (silhueta poligonal, não arte ilustrada), moldura e botões desenhados via
  macroquad.
- **Fonte:** `Press Start 2P` (SIL Open Font License, gratuita), baixada e
  embutida como asset (`crates/grammar_quest/assets/fonts/PressStart2P-Regular.ttf`),
  carregada via `macroquad::text::load_ttf_font_from_bytes`. Licença
  registrada em `ATTRIBUTION.md`, seguindo o padrão já usado para os outros
  assets do projeto.

## Máquina de estados e navegação

### Novos `ScreenMode`

```rust
pub enum ScreenMode {
    MainMenu,   // novo — tela inicial
    Options,    // novo — alcançável só a partir do MainMenu
    Laboratory,
    Playing,
    Won,
}
```

`AppState::new()` passa a iniciar em `ScreenMode::MainMenu` em vez de
`ScreenMode::Laboratory`.

### Sub-estado do menu (não é um `ScreenMode` novo)

Dentro de `ScreenMode::MainMenu`, um campo local (`MainMenuStage` ou um
`bool`/`enum` simples em `AppState`, ex. `menu_stage: MainMenuStage { Root,
ChooseMode }`) controla se a tela mostra os 3 botões raiz (Jogar / Opções /
Sair) ou o sub-passo de escolha de modo (Livre / Enigma) depois de clicar em
Jogar. Isso evita inflar `ScreenMode` com um estado que não precisa de
transição própria no restante do app (nada em `gameplay.rs`/`render/`
depende de saber se o menu está no passo raiz ou no sub-passo).

### Fluxo completo

```text
MainMenu (Root)
  ├─ "▶ Jogar"   → MainMenu (ChooseMode)
  │                   ├─ "🗺 Livre"  → play_mode = Free;   mode = Laboratory
  │                   └─ "🧩 Enigma" → play_mode = Enigma; mode = Laboratory
  ├─ "Opções"    → mode = Options
  └─ "Sair"      → encerra o processo (equivalente ao `break` atual)

Options
  └─ "← Voltar"  → mode = MainMenu (Root)

Laboratory (mode-only, ver seção seguinte)
  └─ Esc         → mode = MainMenu (Root)   [MUDANÇA: hoje Esc fecha o app]

Playing / Won
  └─ Esc         → mode = Laboratory   [sem mudança]
```

Sair do app deixa de ser possível escondido dentro do Laboratory — fica
sempre um passo à frente do Menu, nunca implícito.

## Laboratory reformulado por modo

Hoje `ui/editor.rs` desenha um toggle "🗺 Livre / 🧩 Enigma" e muda o CTA
conforme a seleção (`show_editor`, linhas ~150-280 conforme a versão atual).
Esse toggle é removido. Em vez disso:

- `show_editor` lê `state.play_mode` (já setado pelo menu) e renderiza
  **apenas** a seção daquele modo: formulário de gramática + "Gerar
  sentença aleatória" + "▶ JOGAR NO LABIRINTO 2D" para `Free`; formulário +
  seletor de dificuldade + CTA equivalente para `Enigma`.
- Um rótulo discreto (ex. "Modo Livre" / "Modo Enigma") substitui o toggle
  visualmente, deixando claro em qual modo o jogador está.
- Trocar de modo exige voltar ao menu (Esc) e escolher de novo — não há
  mais troca direta dentro do Laboratory. Isso é intencional (decisão já
  validada com o usuário).
- A ficha `G = {N,T,P,S}` (T1), o painel lateral compartilhado (`side_panel.rs`)
  e toda a lógica de `start_free_maze`/`start_difficulty_maze` (incluindo a
  validação de produtividade/regex do T2) continuam exatamente como estão —
  esta mudança é só de composição visual em `editor.rs`, não de lógica de
  estado.

## Opções

Tela simples com dois blocos:

1. **Volume:** `AppState.master_volume: f32` (padrão `1.0`, clamped
   `0.0..=1.0`), ajustado por um slider egui. `audio::play_once` passa a
   receber o volume como parâmetro explícito em vez do `1.0` fixo atual;
   cada `Sfx::play_*` (`play_door_correct`, `play_door_wrong`,
   `play_victory`, `play_click`) ganha um parâmetro `volume: f32` que os
   call sites em `gameplay.rs`/`ui/*.rs` preenchem com `state.master_volume`.
   Sem estado interno mutável em `Sfx` — o volume atual sempre vem de
   `AppState`, single source of truth. **Não persiste entre execuções** —
   reseta para `1.0` a cada abertura do jogo, consistente com o YAGNI de
   "sem save/load" já declarado em `ROADMAP.md`.
2. **Créditos:** o conteúdo de `crates/grammar_quest/assets/ATTRIBUTION.md`
   (ou o caminho correto do arquivo já existente) embutido via `include_str!`
   e mostrado num `egui::ScrollArea` — sem duplicar o texto em dois lugares.

Botão "← Voltar" retorna a `ScreenMode::MainMenu` (Root).

## Assets novos

- `crates/grammar_quest/assets/fonts/PressStart2P-Regular.ttf` — baixado de
  `https://raw.githubusercontent.com/google/fonts/main/ofl/pressstart2p/PressStart2P-Regular.ttf`,
  SIL Open Font License. Entrada nova em `ATTRIBUTION.md` com a licença.
- Nenhum asset de imagem novo — mascote/moldura/botões são desenhados via
  primitivas do macroquad (retângulos, polígonos via `draw_triangle`/
  `draw_poly`, texto com a fonte carregada).

## Fora de escopo (explicitamente)

- Persistência de volume/configurações entre execuções.
- Animação do mascote (idle/walk) — fica como forma estática ou com
  variação simples de escala/opacidade, não um sprite animado quadro a
  quadro.
- Qualquer alteração em `grammar_engine`.
- T6/T7/T8 do backlog de auditoria (não relacionados a este pedido).
- Reaproveitar o logo neon (`grammar-quest-logo.png`) ou o sprite do
  swordsman no menu — decisão explícita do usuário de manter fidelidade
  total ao mockup em vez disso.

## Testes

- `grammar_quest`: novo teste cobrindo `AppState::new()` inicia em
  `ScreenMode::MainMenu`; transição `MainMenu -> Laboratory` seta
  `play_mode` corretamente para cada escolha; `Esc` em `Laboratory` retorna
  a `MainMenu` (não encerra mais o processo, o que aliás não é testável
  diretamente — testar a troca de `mode`, o encerramento real do processo
  via "Sair" só é verificável manualmente); `master_volume` fica clamped em
  `[0.0, 1.0]`.
- `grammar_engine`: nenhuma mudança, suíte existente deve continuar
  passando inalterada (fonte de verdade dos requisitos 1-4 e 7).
- Verificação manual obrigatória via `cargo run -p grammar_quest` — mudança
  de tela/visual não é totalmente coberta por testes automatizados.
- `cargo build --workspace`, `cargo clippy --workspace --all-targets`,
  `cargo test --workspace` devem passar sem avisos novos ao final.

# Grammar Quest — Fase 1: App Shell e Painel de Derivação

Data: 2026-09-14  
Status: aprovado para especificação; aguarda revisão do documento antes do plano

## Objetivo

Entregar a primeira tela funcional do Grammar Maze: uma aplicação desktop que
permite escolher ou editar uma gramática regular, gerar uma sentença aleatória
e visualizar toda a evidência acadêmica da derivação — sentença, expressão
regular, produções aplicadas e instantâneos da pilha. Esta fase atende os
requisitos do trabalho sem antecipar o labirinto jogável da Fase 2.

## Decisões confirmadas

- Layout **Workspace com painel lateral**: editor à esquerda, resultado em
  destaque no espaço principal e rastreio da pilha/derivação na lateral.
- Entrada é um editor textual (`S -> aS | ab`) acompanhado dos três presets
  embutidos; não haverá campos separados para N, T, P e S nesta fase.
- Direção visual **Neon Arcade**: fundo quase preto com gradiente roxo sutil,
  ciano para ação e sentença ativa, magenta para seleção e verde reservado a
  sucesso. O brilho é pontual, nunca usado como único meio de comunicar estado.
- Gerar atualiza a derivação inteira de uma vez. Animações de push/pop e o
  labirinto interativo pertencem, respectivamente, às Fases 3 e 2.

## Arquitetura

`grammar_engine` continua sendo a única fonte de lógica formal. A crate
`grammar_quest` cria um estado de tela testável e adapta os dados públicos do
motor para a apresentação egui; ela não reimplementa parsing, validação,
pilha, derivação nem regex.

```
Editor de texto ou preset
        │
        ▼
AppState::generate()
        │
        ├── parse_grammar(text)
        ├── validate_regular(&grammar)
        ├── derive_random(&grammar)
        └── to_regex(&grammar)
        ▼
PanelResult | mensagem de erro
        ▼
editor (entrada/erro) + workspace (sentença/regex) + side panel (passos/pilha)
```

### Módulos da crate `grammar_quest`

- `state.rs`: `AppState`, `PanelResult` e a operação `generate`. É a camada
  pura/testável que orquestra a API pública de `grammar_engine` e mantém
  texto, preset, `Option<PanelResult>` e `Option<String>` para erro em
  português.
- `ui/editor.rs`: editor multilinha, seletor dos presets e botão de geração.
  Só transforma interação egui em mutações de `AppState`.
- `ui/side_panel.rs`: mostra, em ordem, cada produção aplicada, a pilha logo
  após o push e a saída parcial. Não importa macroquad nem altera o estado.
- `ui/theme.rs`: paleta Neon Arcade e helpers egui para contornos, superfícies
  e estados de foco/hover; centraliza os valores de cor.
- `main.rs`: configura macroquad, desenha o fundo e compõe o workspace egui.

## Estado e interação

Ao abrir, o editor contém `S -> aS | ab`, o preset “Exemplo do professor” está
selecionado e ainda não há resultado. Selecionar outro preset substitui o texto
do editor e limpa resultado/erro. Alterar manualmente o texto limpa
resultado/erro e marca o preset como “Editor manual”.

Ao clicar em “Gerar sentença aleatória”, o estado aplica parse, validação,
derivação e conversão nessa ordem. Um sucesso substitui o resultado inteiro;
um erro mostra a mensagem portuguesa do motor junto ao editor e não deixa um
resultado antigo aparentar ser fruto do novo texto. O resultado contém a
sentença final, regex e os `DerivationStep` já produzidos pelo motor.

## Layout e linguagem visual

- Janela 1280×800 já configurada, com espaço mínimo e responsividade simples:
  quando faltar largura, o painel lateral ocupa uma largura fixa menor e suas
  listas rolam em vez de comprimir o texto.
- Cabeçalho discreto: marca “Grammar Maze”, indicador “Laboratório de
  Gramáticas Regulares” e dica `Esc` para sair.
- Coluna esquerda: seletor de presets, editor e botão ciano de geração.
- Área central: card de resultado com a sentença grande e a regex abaixo;
  antes de gerar, exibe uma instrução curta de primeiro uso.
- Lateral direita: cartões de passo numerados; cada cartão apresenta
  `A → α`, saída parcial e a pilha em ordem topo→base.
- Ciano e magenta só destacam controles e foco; verde representa sucesso e
  mensagens de erro usam texto/ícone além de cor para acessibilidade.

A hierarquia intencional prioriza a ação de gerar, depois a sentença/regex e
por fim a trilha detalhada. Essa escolha segue recomendações de usar contraste,
escala e agrupamento para guiar a leitura em interfaces densas
([NN/g: Visual Hierarchy](https://www.nngroup.com/articles/visual-hierarchy-ux-definition/)).

## Tratamento de erros

Não haverá `unwrap`/`expect` para dados do usuário. `GrammarError` é convertido
em texto via `Display` e mostrado inline. Uma gramática malformada, não regular,
improdutiva ou sem conversão válida mantém a aplicação utilizável: o usuário
edita e tenta novamente sem reiniciar a janela.

## Verificação

- Testes unitários em `state.rs` comprovam: carregamento de preset, limpeza ao
  editar, fluxo completo do exemplo do professor e erro de gramática inválida.
- `cargo test`, `cargo fmt -- --check`, `cargo clippy --workspace --all-targets
  -- -D warnings` e `cargo build` precisam passar.
- Verificação manual: `cargo run -p grammar_quest`; conferir presets, edição
  inválida, geração repetida e os dados visíveis de pilha, passos, sentença e
  regex.

## Fora de escopo

- Escolha manual de produção, `derive_step` e corredores do labirinto.
- Animações temporizadas, som, pontuação e persistência.
- Dependências novas; a fase usa apenas macroquad, egui-macroquad e
  `grammar_engine`, já presentes no workspace.

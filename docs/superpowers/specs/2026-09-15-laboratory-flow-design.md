# Design: Fluxo de Preparação do Labirinto

Data: 2026-09-15

## Objetivo

Substituir completamente o laboratório de três colunas mostrado na referência
do usuário. A nova tela deixa de parecer uma ferramenta técnica aberta o tempo
todo e passa a ser uma sequência de menus de preparação, usando a mesma
linguagem retrô 16-bit da tela inicial.

O motor de gramática não muda. Os requisitos acadêmicos continuam visíveis e
acessíveis: entrada de produções, três exemplos, geração aleatória,
derivação por pilha e expressão regular. Eles apenas aparecem no passo em que
são necessários, não como três painéis permanentes.

## Fluxo

```text
Menu principal
  └─ Jogar → Livre / Enigma

Livre
  └─ Preparação livre
       ├─ Escolher exemplo → seleção das 3 gramáticas → preparação livre
       ├─ Criar gramática → editor de produções → preparação livre
       ├─ Gerar sentença → resultado formal
       └─ Jogar labirinto → Playing

Enigma
  └─ Preparação do enigma
       ├─ escolher dificuldade
       └─ Iniciar enigma → Playing

Resultado formal
  ├─ sentença, G={N,T,P,S}, pilha/derivação e regex
  ├─ Jogar labirinto livre
  └─ Voltar à preparação livre
```

`Esc` retorna da preparação ao menu principal; no editor e no resultado,
retorna à preparação do modo atual. O retorno de Playing/Won continua indo
para a preparação, não para o antigo painel triplo.

## Estado

`ScreenMode` permanece pequeno. Um novo subestado em `AppState` modela as
variações internas do laboratório:

```rust
pub enum LaboratoryStage {
    Setup,
    GrammarEditor,
    FormalResult,
}
```

Entrar pelo menu seleciona `PlayMode` e inicia em `LaboratoryStage::Setup`.
Escolher ou editar uma gramática mantém a fonte de verdade atual
(`grammar_text`, `selected_example`, `grammar_preview`). Gerar uma sentença
usa a implementação atual de `AppState::generate()` e abre `FormalResult`
somente em caso de sucesso. Erros de parsing/regularidade continuam no estado
e são exibidos no editor.

## Telas

### Preparação livre

Uma única coluna central, sobre o fundo roxo com scanlines da abertura. Um
cabeçalho curto identifica `Modo Livre`; três botões largos, com moldura
pixelada, levam a `Escolher exemplo`, `Criar gramática` e `Gerar sentença`.
O exemplo ativo aparece como texto de status abaixo do primeiro botão. O CTA
`Jogar labirinto` só ocupa o destaque principal quando a gramática atual for
regular; se não for, mostra a mensagem de correção em vez de iniciar.

### Seleção e edição de gramática

`Escolher exemplo` abre três opções grandes, uma por gramática embutida, com
nome, descrição e produções resumidas. `Criar gramática` abre o campo
multilinha de produções e a ficha viva `G={N,T,P,S}`. Esse é o único ponto em
que o textarea e os detalhes formais ocupam espaço amplo. Salvar/voltar
retorna à preparação e não descarta texto válido digitado.

### Preparação do enigma

Cinco cartões de dificuldade substituem o combobox: Fácil, Médio, Difícil,
Extremo e Impossível, cada um com faixa de portas. Um cartão selecionado ganha
borda dourada; `Iniciar enigma` gera a gramática e inicia o labirinto como
hoje. Não apresenta editor ou exemplos que o modo Enigma não utiliza.

### Resultado formal e trilha no jogo

Após geração no modo Livre, `FormalResult` mostra a sentença, a ficha
`G={N,T,P,S}`, os passos da derivação por pilha e a regex com eliminação de
variáveis. Assim a demonstração continua completa e é mais direta para o
avaliador. Durante o labirinto, o botão existente de trilha mantém o painel
compartilhado `side_panel.rs`; não haverá um painel direito permanente na
preparação.

## Organização de código

- `ui/laboratory.rs`: composição dos estágios Setup/Editor/Resultado e os
  pequenos enums de ação de UI.
- `ui/side_panel.rs`: permanece dono da visualização de derivação compartilhada
  durante o labirinto; perde apenas o uso permanente na preparação.
- `ui/editor.rs`: torna-se o formulário sob demanda, sem cabeçalho de painel
  ou CTAs de modo.
- `app.rs`: despacha `LaboratoryStage` e converte ações de UI em métodos de
  `AppState`; continua dono da criação de `Room` e `Player`.
- `state.rs`: adiciona transições explícitas entre setup, editor e resultado.

## Critérios de aceite

1. A abertura do modo não exibe o painel triplo da referência.
2. Livre permite selecionar as três gramáticas ou digitar produções e iniciar
   o labirinto com a gramática escolhida.
3. Enigma apresenta cartões de dificuldade, não o editor de gramática.
4. A derivação completa, `G={N,T,P,S}` e regex são acessíveis após gerar uma
   sentença e dentro do labirinto, sem duplicar lógica do motor.
5. `grammar_engine` não recebe dependência de UI e toda a suíte do workspace
   continua verde.

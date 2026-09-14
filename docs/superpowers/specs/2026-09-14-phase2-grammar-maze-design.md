# Grammar Quest — Fase 2: Grammar Maze Jogável

## Objetivo

Transformar a derivação de uma gramática regular em um jogo top-down: cada
bifurcação representa as alternativas do não-terminal atual, atravessar uma
porta aplica a produção escolhida à pilha real e o nível termina quando a pilha
esvazia.

## Decisões aprovadas

- Mapa 2D top-down com avatar, câmera e corredores físicos.
- A porta é a escolha: cada corredor mostra uma produção válida e atravessá-lo
  aplica essa produção.
- O mapa cresce à frente do jogador após cada escolha; não há percurso falso
  nem labirinto pré-definido desconectado da gramática.
- Terminais são consumidos automaticamente entre bifurcações.
- O avatar começa como um placeholder isolado em `assets/player/`, pronto para
  ser substituído pela arte que o usuário fornecer.

## Arquitetura

`grammar_engine` recebe um estado controlável de derivação: inicialização,
consumo de terminais pendentes e aplicação de uma alternativa por índice. O
modo aleatório existente passa a reutilizar essa mesma base, trocando somente
quem escolhe o índice (RNG ou o jogador).

`grammar_quest` adiciona `GameState` (Laboratory, Playing, Won), um módulo
`maze` para geometria/portas/colisão e um módulo `player` para movimento e
renderização. O painel lateral existente recebe o mesmo estado de derivação,
sem duplicar regras formais.

## Fluxo

1. O jogador gera ou seleciona uma gramática no laboratório e inicia o maze.
2. O jogo cria uma sala para o não-terminal no topo da pilha e uma porta por
   alternativa.
3. Atravessar uma porta chama `apply_choice`, registra o passo e consome os
   terminais que passarem ao topo.
4. Se houver não-terminal, a próxima sala é construída à frente; se a pilha
   esvaziar, a saída e a tela de vitória aparecem.

## Experiência visual

- Neon Arcade top-down: piso escuro, trilhas ciano, placas magenta e efeitos
  verdes de confirmação.
- Câmera segue o avatar, com corredores curtos e 2–3 escolhas bem legíveis.
- HUD compacto apresenta saída parcial, pilha e regex; o painel detalhado é
  reaproveitado e recolhível.
- Vitória mostra sentença e regex, com “Jogar novamente” e voltar ao laboratório.

## Regras e segurança

- Somente alternativas reais da gramática são portas disponíveis.
- Erros do motor retornam à UI como mensagem, nunca panic.
- Não adicionar dependências sem necessidade; macroquad fornece input,
  desenhos, colisão simples e câmera.

## Verificação

- Testes unitários no motor para escolhas controladas, consumo de terminais e
  estado final.
- Testes puros para geração de portas a partir das alternativas.
- `cargo test`, fmt, Clippy e build passam.
- Verificação manual: completar cada preset, tentar todas as portas de uma
  sala e confirmar a atualização de painel, câmera e vitória.

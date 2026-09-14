# Roadmap — Grammar Quest (Grammar Maze)

Cada fase entrega algo rodável. As fases 0+1 sozinhas já cumprem 100% da
especificação do trabalho — servem de rede de segurança caso o tempo
aperte antes da Fase 2/3.

## Fase 0 — Motor de gramática (`grammar_engine`)

- [ ] Modelar `Grammar { non_terminals, terminals, productions, start }`.
- [ ] Parser de uma notação textual simples (`S -> aS | ab`).
- [ ] Validação: gramática é regular (produções só do tipo `A -> aB`,
      `A -> a` ou `A -> ε`).
- [ ] Derivação aleatória via **pilha** (`Vec<Symbol>`), seguindo o
      algoritmo do PDF passo a passo, registrando cada produção aplicada.
- [ ] Conversão para expressão regular por eliminação de variáveis
      não-terminais (equações à la exemplo do PDF: `S = aS + ab` → `a*ab`).
- [ ] Três gramáticas de exemplo embutidas.
- [ ] Testes unitários cobrindo o exemplo exato do PDF (`S ::= aS | ab` →
      regex `a*ab`).

## Fase 1 — Casca do app + painel (`grammar_quest`)

- [ ] Janela macroquad + overlay egui-macroquad.
- [ ] Formulário de entrada da gramática (N, T, P, S) ou seletor das 3
      gramáticas prontas.
- [ ] Botão "gerar sentença aleatória" chamando `grammar_engine`.
- [ ] Painel lateral: pilha (estado atual), log de derivação passo a
      passo, sentença final, expressão regular.
- [ ] Neste ponto: **especificação do professor 100% atendida.**

## Fase 2 — Grammar Maze

- [ ] Tela de labirinto reaproveitando o painel lateral da Fase 1.
- [ ] Bifurcações do labirinto = produções do não-terminal atual;
      escolher um corredor = aplicar aquela produção (empilha/desempilha
      de verdade, ao vivo, refletido no painel).
- [ ] Chegar a uma produção só-terminal fecha a pilha e libera a saída.
- [ ] Pelo menos as 3 gramáticas de exemplo devem ser jogáveis como
      labirinto.

## Fase 3 — Polish ("fluido e bonito")

- [ ] Animação de push/pop da pilha (tween, não corte seco).
- [ ] Log de derivação com efeito de digitação.
- [ ] Tema visual coeso (paleta, tipografia, transições entre telas).
- [ ] Portas-puzzle: sentenças falsas geradas e validadas contra a regex
      derivada (garantia de que as opções erradas realmente não pertencem
      à linguagem).
- [ ] Tela de vitória / pontuação.

## Fora de escopo (YAGNI por enquanto)

- Gramáticas livres de contexto ou sensíveis ao contexto.
- Multiplayer, save/load, sons.
- Qualquer ferramenta de indexação/grafo de código externa.

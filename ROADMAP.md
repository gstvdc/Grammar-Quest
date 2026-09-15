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

## Fora de escopo (YAGNI por enquanto)

- Gramáticas livres de contexto ou sensíveis ao contexto.
- Multiplayer, save/load, sons.
- Qualquer ferramenta de indexação/grafo de código externa.

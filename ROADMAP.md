# Direções futuras do kaju

O kaju 1.0 já é uma linguagem completa: tipos, funções e closures, orientação a objetos, exceções, módulos, coleções com métodos, testes embutidos (`afirme` e `kaju teste`) e diagnósticos ricos em português.

Este documento descreve **para onde o kaju pode crescer**. Não é um cronograma nem uma promessa de datas — é a direção. As ideias estão agrupadas por tema, aproximadamente em ordem de interesse. Contribuições e discussões são bem-vindas (veja o [guia de contribuição](CONTRIBUTING.md)).

## Linguagem

- **Sintaxe curta para funções anônimas** — uma forma enxuta de escrever funções passadas a `.mapeie`, `.filtre` e `.reduza`, além da forma completa `funcao(x) { ... }`.
- **Correspondência de padrões** — evoluir o `escolha` para desestruturar listas, dicionários e objetos, não só comparar valores.
- **Contratos entre classes** — uma forma de declarar que classes diferentes compartilham a mesma interface, complementando a herança.
- **Iteração preguiçosa** — sequências que produzem valores sob demanda, para trabalhar com fluxos grandes sem materializar tudo em memória.

## Biblioteca padrão

- Ampliar os módulos de texto, matemática, data/hora e coleções conforme as necessidades reais aparecem.
- Mais utilidades de entrada e saída e de manipulação de arquivos.

## Ferramentas

- **`kaju formatar`** — um formatador oficial que reescreve o código num estilo canônico (preservando comentários), para acabar com discussões de estilo.
- **Servidor de linguagem (LSP)** — autocompletar, ir para a definição e erros em tempo real nos editores, além do realce de sintaxe que já existe.
- **Depurador** — inspecionar variáveis e acompanhar a execução passo a passo.

## Alcance e distribuição

- **Experimente no navegador** — compilar o interpretador para WebAssembly e oferecer um ambiente onde qualquer pessoa roda kaju sem instalar nada.
- **Mais plataformas nos binários prontos** — cobrir também Linux ARM (`aarch64`), além dos alvos atuais.

## Desempenho

- **Máquina virtual de bytecode** — hoje o kaju interpreta a árvore sintática diretamente. Compilar para bytecode e executar numa VM (nos moldes do *Crafting Interpreters*) daria um salto de desempenho, mantendo a mesma linguagem e a mesma biblioteca.

---

Tem uma ideia que não está aqui? Abra uma *issue* para conversarmos.

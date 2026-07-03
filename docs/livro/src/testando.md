# Testando

O kaju vem com um executor de testes embutido: você escreve os testes na própria linguagem e roda todos com um comando. A base é a função `afirme`, apresentada no capítulo de [erros](./erros.md).

## A ideia

Um teste é uma **função sem parâmetros cujo nome começa com `teste`**. Dentro dela, você usa `afirme` para declarar o que espera. O teste **passa** se a função rodar até o fim; **falha** se qualquer `afirme` (ou qualquer outro erro) acontecer.

```kaju
funcao soma(a, b) {
    retorne a + b
}

funcao teste_soma_simples() {
    afirme(soma(2, 3) == 5)
    afirme(soma(-1, 1) == 0, "soma com negativo")
}

funcao teste_listas_iguais() {
    afirme([1, 2, 3] == [1, 2, 3])
    afirme([1, 2] != [2, 1])
}
```

Funções que **não** começam com `teste` são ignoradas pelo executor — use-as livremente como código auxiliar (no exemplo, `soma`).

## Rodando os testes

Use o comando `kaju teste`, passando um arquivo ou uma pasta:

```bash
kaju teste exemplo_teste.kaju   # roda um arquivo
kaju teste testes/              # roda uma pasta (busca recursiva)
kaju teste                      # roda a pasta atual (.)
```

Quando você passa uma **pasta**, o kaju procura arquivos `.kaju`/`.kj` que tenham **`teste` no nome** (por exemplo, `matematica_teste.kaju`). Quando você passa um **arquivo** diretamente, ele é executado independentemente do nome.

Para cada arquivo, o kaju primeiro executa o código de cima para baixo — o que serve para preparar o cenário — e depois chama cada função `teste*`, em ordem alfabética.

## A saída

```
testes/exemplo_teste.kaju
  ✓ teste_listas_iguais
  ✓ teste_soma_simples

resumo: 2 passaram, 0 falharam
```

Cada teste vira uma linha com `✓` (passou) ou `✗` (falhou). Quando um teste falha, o kaju mostra logo abaixo o código e a mensagem do erro:

```
matematica_teste.kaju
  ✓ teste_soma_simples
  ✗ teste_quociente
      K231: afirmação falhou: 10 / 3 deveria ser inteiro?

resumo: 1 passaram, 1 falharam
```

No fim, o comando **encerra com um código de erro** se algum teste falhar (ou se algum arquivo não puder ser lido ou executado). É isso que permite usá-lo em integração contínua: o processo "quebra" quando um teste falha, e passa quando tudo está verde.

## Organizando os testes

Uma convenção simples e comum: guarde os testes numa pasta `testes/`, com um arquivo por área do seu programa.

```
meu_projeto/
├── principal.kaju
├── matematica.kaju
└── testes/
    ├── matematica_teste.kaju
    └── texto_teste.kaju
```

Como cada arquivo é executado antes dos testes, você pode `importe` o módulo que quer testar no topo do arquivo de teste (veja [Módulos](./modulos.md)) e então afirmar sobre as funções e classes que ele exporta.

```kaju
importe "../matematica.kaju"

funcao teste_area() {
    afirme(areaRetangulo(2, 3) == 6)
}
```

A seguir: [Suporte a editor](./editor.md).

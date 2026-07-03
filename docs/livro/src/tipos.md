# Valores e tipos

Todo dado em um programa kaju é um **valor**, e todo valor tem um **tipo**. A função `tipo(x)` devolve o nome do tipo de qualquer valor, como texto.

| Tipo | O que é | Exemplos |
|------|---------|----------|
| `numero` | Números inteiros e decimais | `42`, `-7`, `3.14` |
| `texto` | Cadeias de caracteres | `"caju"`, `"olá"` |
| `logico` | Verdadeiro ou falso | `verdadeiro`, `falso` |
| `lista` | Sequência ordenada de valores | `[1, 2, 3]` |
| `dicionario` | Mapa de chaves para valores | `{"nome": "Ana"}` |
| `funcao` | Uma função | `funcao(x) { retorne x }` |
| `nulo` | Ausência de valor | `nulo` |

```kaju
escreva(tipo(42))          // numero
escreva(tipo("oi"))        // texto
escreva(tipo(verdadeiro))  // logico
escreva(tipo([1, 2]))      // lista
```

O kaju tem **tipagem dinâmica**: uma variável não é presa a um tipo, e o tipo de um valor é conhecido enquanto o programa roda. Você não declara tipos; eles surgem dos próprios valores.

## Números: inteiros e decimais

Há um único tipo visível, `numero` — `tipo(x)` sempre responde `"numero"`. Por baixo, porém, o kaju distingue **inteiros** de **decimais**, para oferecer precisão exata quando ela importa:

```kaju
escreva(2 + 2)              // 4         (inteiro exato)
escreva(10 / 3)             // 3.3333333333333335   (decimal)
escreva(10 / 2)             // 5.0       (divisão sempre dá decimal)
escreva(0.1 + 0.2)          // 0.30000000000000004
escreva(9007199254740993)   // exato, sem perder precisão
```

As regras que governam essa distinção:

- Um literal **sem** ponto é inteiro (`5`); **com** ponto é decimal (`5.0`).
- `+`, `-` e `*` entre inteiros produzem um inteiro exato; se qualquer lado é decimal, o resultado é decimal.
- `/` (divisão) sempre produz decimal, mesmo entre inteiros (`10 / 2` dá `5.0`).
- `%` (resto) dá inteiro entre inteiros e decimal caso contrário.
- Comparações são matemáticas: `5 == 5.0` é `verdadeiro`.
- Ao imprimir, decimais mostram o ponto (`5.0`) para se distinguir dos inteiros (`5`).
- Se uma operação com inteiros (`+`, `-`, `*`) ultrapassa o limite do inteiro, o kaju interrompe com o erro `K222` em vez de perder precisão silenciosamente; para valores muito grandes, trabalhe com decimais (`9223372036854775807.0 + 1.0`).

> Não existe operador de divisão inteira com `//` (essa sequência inicia um comentário). Para obter a parte inteira de uma divisão, use `piso(a / b)`.

## Textos

Um `texto` é uma cadeia de caracteres entre aspas duplas, codificada em UTF-8 — acentos e outros caracteres são bem-vindos:

```kaju
escreva("caju")
escreva("programação")
```

O texto ganha um capítulo próprio mais adiante; por ora, basta saber que é o tipo dos valores textuais.

## Verdadeiro e falso

O tipo `logico` tem exatamente dois valores: `verdadeiro` e `falso`. Eles aparecem em comparações e controlam decisões no programa.

Quando um valor de outro tipo é usado como condição, o kaju precisa decidir se ele conta como verdadeiro ou falso. A regra é simples: apenas `falso` e `nulo` são considerados "falsos". **Todo o resto é verdadeiro**, inclusive `0` e o texto vazio `""`.

```kaju
se 0 { escreva("zero conta como verdadeiro") }   // isto imprime
se nulo { escreva("isto não imprime") }
```

## Nulo

`nulo` representa a ausência de um valor. É o que uma função devolve quando não retorna nada, e o único valor, além de `falso`, que conta como falso numa condição.

Próximo: [Variáveis e operadores](./variaveis.md)

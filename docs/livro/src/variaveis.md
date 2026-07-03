# Variáveis e operadores

## Declarando variáveis

Uma variável dá nome a um valor. Use `var` para variáveis que podem mudar e `constante` para valores que permanecem fixos:

```kaju
var contador = 0
contador = contador + 1
escreva(contador)          // 1

constante PI = 3.14159
// PI = 4    // erro K009: não é possível reatribuir uma constante
```

Reatribuir uma `constante` é um erro relatado pelo interpretador. Isso deixa explícito, para quem lê o programa, que aquele valor não se altera.

## Atribuição composta

Para alterar uma variável a partir do próprio valor, há atalhos:

```kaju
var x = 10
x += 5     // x = x + 5  ->  15
x -= 3     // 12
x *= 2     // 24
x /= 4     // 6.0
x %= 4     // 2.0
```

Esses operadores também funcionam em índices e campos: `lista[0] += 1`, `objeto.contador += 1`.

## Desempacotamento

Uma única atribuição pode dar nome a vários valores de uma vez:

```kaju
var a, b = 1, 2        // a = 1, b = 2
a, b = b, a            // troca os dois: a = 2, b = 1
var x, y, z = [10, 20, 30]   // desempacota os itens de uma lista
```

A troca `a, b = b, a` funciona porque o lado direito é avaliado por inteiro antes de qualquer atribuição.

## Operadores aritméticos

```kaju
escreva(7 + 2)    // 9
escreva(7 - 2)    // 5
escreva(7 * 2)    // 14
escreva(7 / 2)    // 3.5
escreva(7 % 2)    // 1   (resto da divisão)
```

O `+` tem um papel a mais: entre textos, ele **concatena**.

```kaju
escreva("ca" + "ju")   // caju
```

## Comparação

Os operadores `==`, `!=`, `<`, `>`, `<=` e `>=` devolvem um valor `logico`. Funcionam entre números e entre textos, onde comparam pela ordem alfabética:

```kaju
escreva(3 < 5)               // verdadeiro
escreva("abacaxi" < "caju")  // verdadeiro
escreva(5 == 5.0)            // verdadeiro
```

## Operadores lógicos

As conexões lógicas são palavras em português: `e`, `ou` e `nao`. Os operadores `e` e `ou` têm **curto-circuito** — o kaju só avalia o segundo lado quando é necessário para decidir o resultado.

```kaju
var idade = 20
se idade >= 18 e nao (idade > 65) {
    escreva("adulto em idade produtiva")
}
```

## Operador ternário

O operador `? :` escolhe entre dois valores conforme uma condição, dentro de uma única expressão:

```kaju
var rotulo = idade >= 18 ? "adulto" : "menor"
escreva(rotulo)
```

## Lidando com nulos: `??` e `?.`

Dois operadores tornam o trabalho com valores que podem ser `nulo` mais curto e seguro.

O operador `??` (**coalescência de nulo**) devolve o lado esquerdo quando ele não é `nulo`; caso contrário, avalia e devolve o direito. Diferente de `ou`, ele só reage a `nulo` — valores como `0`, `""` ou `falso` passam intactos:

```kaju
var nome = nulo
escreva(nome ?? "convidado")   // convidado
escreva(0 ?? 99)               // 0  (zero não é nulo)
```

O operador `?.` (**acesso opcional**) acessa um campo ou chama um método só se o lado esquerdo não for `nulo`. Se for `nulo`, o resultado é `nulo` — sem erro:

```kaju
escreva(usuario?.endereco?.cidade)   // nulo se qualquer elo for nulo
escreva(texto?.maiusculas())         // nulo se 'texto' for nulo
```

Cada `?.` protege o seu próprio lado esquerdo, então encadear é natural: se um elo intermediário for `nulo`, os `?.` seguintes propagam o `nulo` adiante. Os dois combinam muito bem para fornecer um padrão:

```kaju
var cidade = usuario?.endereco?.cidade ?? "desconhecida"
```

> Se você usar `.` (sem `?`) sobre um valor nulo, isso continua sendo um erro — o `?.` é a forma explícita de dizer "aceite nulo aqui".

## Operadores de bits

Sobre inteiros, o kaju oferece operações bit a bit: `&` (e), `|` (ou), `^` (ou-exclusivo), `~` (não) e os deslocamentos `<<` e `>>`.

```kaju
escreva(5 & 3)     // 1
escreva(5 | 2)     // 7
escreva(1 << 4)    // 16
escreva(~5)        // -6
```

A seguir, um capítulo dedicado aos [textos](./textos.md).

# Controle de fluxo

Até aqui os programas rodaram de cima para baixo, uma linha após a outra. As estruturas de **controle de fluxo** mudam essa ordem: elas decidem quais trechos executar e quantas vezes repeti-los.

Em todas elas, o corpo fica entre chaves `{ }`, e a condição é uma expressão que resulta em verdadeiro ou falso. Vale lembrar a regra de veracidade do kaju: apenas `falso` e `nulo` contam como falsos; qualquer outro valor é considerado verdadeiro.

## Decisão: se, senaose, senao

O `se` executa um bloco somente quando sua condição é verdadeira. Você pode encadear alternativas com `senaose` e fechar com um `senao`, que roda quando nenhuma condição anterior deu certo:

```kaju
se nota >= 90 {
    escreva("A")
} senaose nota >= 80 {
    escreva("B")
} senao {
    escreva("reprovado")
}
```

O kaju testa as condições em ordem e executa apenas o primeiro bloco cuja condição é verdadeira. Tanto os `senaose` quanto o `senao` são opcionais: um `se` sozinho já é válido.

Para escolher um valor entre dois de forma curta, existe o **operador condicional** (ou ternário), na forma `condicao ? valorSeVerdadeiro : valorSeFalso`:

```kaju
var rotulo = idade >= 18 ? "adulto" : "menor"
```

## escolha entre vários casos

Quando você compara um mesmo valor com muitas possibilidades, `escolha` costuma ser mais claro que uma longa sequência de `senaose`. Cada `caso` lista um ou mais valores; assim que um deles casa, seu bloco roda — e apenas ele, pois não há "vazamento" para os casos seguintes. O bloco `padrao` é opcional e roda quando nenhum caso casa:

```kaju
escolha dia {
    caso 1 { escreva("domingo") }
    caso 2, 3, 4, 5, 6 { escreva("dia útil") }   // vários valores no mesmo caso
    caso 7 { escreva("sábado") }
    padrao { escreva("inválido") }
}
```

`escolha` funciona tanto com números quanto com textos.

## Repetição: enquanto

O `enquanto` repete seu bloco enquanto a condição continuar verdadeira. Ele testa a condição antes de cada volta:

```kaju
var i = 0
enquanto i < 3 {
    escreva(i)      // 0, 1, 2
    i += 1
}
```

Cuide para que algo dentro do bloco acabe tornando a condição falsa — aqui, `i += 1` faz `i` crescer até chegar a 3 e parar. Caso contrário, o laço repete para sempre.

## Repetição: para (contagem)

Quando você já sabe o intervalo de números a percorrer, o `para` é mais direto. Escreva `para VAR de INICIO ate FIM`: a variável assume cada inteiro de `INICIO` até `FIM`, e o valor final **é incluído**:

```kaju
para i de 1 ate 5 {
    escreva(i)      // 1, 2, 3, 4, 5
}
```

Por padrão a variável avança de 1 em 1. Para mudar esse ritmo, acrescente `passo` seguido do incremento. Um passo negativo faz uma **contagem regressiva** — nesse caso o `de` é maior que o `ate`:

```kaju
para i de 0 ate 10 passo 2 {
    escreva(i)      // 0, 2, 4, 6, 8, 10
}

para i de 10 ate 1 passo -1 {
    escreva(i)      // 10, 9, 8, ..., 1
}
```

O passo não pode ser zero — um laço que nunca avança nunca terminaria, então o kaju o rejeita com o erro K205.

## Repetição: para cada (iteração)

Para percorrer os elementos de uma lista, use `para cada VAR em COLECAO`. A variável recebe, a cada volta, um elemento da coleção:

```kaju
para cada fruta em ["caju", "manga"] {
    escreva(fruta)
}
```

A mesma forma percorre um dicionário; nesse caso, a variável recebe cada chave.

## Interrompendo um laço: pare e continue

Dentro de qualquer laço, dois comandos ajustam o fluxo: `pare` encerra o laço imediatamente, e `continue` abandona a volta atual e segue para a próxima:

```kaju
para i de 1 ate 10 {
    se i == 3 { continue }   // pula o 3
    se i > 5 { pare }        // encerra ao passar de 5
    escreva(i)               // 1, 2, 4, 5
}
```

Note que `pare` e `continue` só fazem sentido dentro de um laço. Usá-los fora de um laço (ou usar `retorne` fora de uma função) gera o erro K016.

A seguir, [Coleções: listas e dicionários](./colecoes.md).

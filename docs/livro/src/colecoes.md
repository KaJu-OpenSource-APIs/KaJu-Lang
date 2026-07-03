# Coleções: listas e dicionários

Uma variável guarda um único valor. Para trabalhar com vários valores de uma vez, o kaju oferece duas coleções: a **lista**, uma sequência ordenada de valores, e o **dicionário**, um conjunto de pares chave-valor.

## Listas

Uma lista é escrita entre colchetes `[ ]`, com os elementos separados por vírgula. Cada elemento ocupa uma posição, e as posições começam em `0`. Você acessa um elemento pelo seu índice entre colchetes:

```kaju
var frutas = ["caju", "manga", "acerola"]
escreva(frutas[0])          // caju
frutas[1] = "goiaba"        // substitui o elemento na posição 1
escreva(tamanho(frutas))    // 3
```

A função `tamanho` diz quantos elementos a lista tem.

### Métodos de lista

Assim como os textos, as listas têm métodos chamados com `.`. Alguns modificam a lista no lugar; outros apenas devolvem uma informação:

```kaju
var l = [3, 1, 2]
l.adicione(4)               // acrescenta ao fim: [3, 1, 2, 4]
l.remova(0)                 // remove a posição 0 e devolve o valor removido
l.inverta()                 // inverte a ordem no lugar
l.ordene()                  // ordena no lugar

escreva(l.contem(2))        // verdadeiro
escreva(l.indiceDe(2))      // posição do valor, ou -1 se não houver
escreva(l.fatie(0, 2))      // sublista da posição 0 até antes da 2
escreva([1, 2, 3].junte("-"))  // "1-2-3"  (une os elementos num texto)
escreva([1, 2, 3, 4].soma())   // 10
```

### Transformando listas

Alguns métodos recebem uma **função** como argumento e a aplicam a cada elemento. Eles são muito úteis para transformar ou filtrar dados sem escrever laços manualmente:

- `mapeie(f)` cria uma nova lista aplicando `f` a cada elemento;
- `filtre(f)` cria uma nova lista só com os elementos para os quais `f` devolve verdadeiro;
- `reduza(inicial, f)` combina todos os elementos em um único valor, partindo de `inicial`.

```kaju
var nums = [1, 2, 3, 4, 5]
escreva(nums.mapeie(funcao(x) { retorne x * 2 }))            // [2, 4, 6, 8, 10]
escreva(nums.filtre(funcao(x) { retorne x % 2 == 0 }))       // [2, 4]
escreva(nums.reduza(0, funcao(acc, x) { retorne acc + x }))  // 15
```

Para ordenar por um critério próprio, `ordenePor(f)` usa o valor devolvido por `f` como chave de ordenação:

```kaju
var pessoas = [{"nome": "Ana", "idade": 30}, {"nome": "Beto", "idade": 25}]
pessoas.ordenePor(funcao(p) { retorne p["idade"] })
// agora Beto vem antes de Ana
```

## Dicionários

Um dicionário associa **chaves** a **valores**. Pense nele como uma agenda: você procura pela chave (um texto) e obtém o valor guardado. Ele é escrito entre chaves `{ }`, com cada par no formato `"chave": valor`:

```kaju
var pessoa = {"nome": "Ana", "idade": 30}
escreva(pessoa["nome"])         // Ana
pessoa["idade"] = 31            // altera o valor de uma chave existente
pessoa["cidade"] = "Recife"     // uma chave nova é criada ao ser atribuída
```

Você acessa e altera valores indexando pela chave, entre colchetes. Atribuir a uma chave que ainda não existe a cria.

Os dicionários também têm métodos:

```kaju
escreva(pessoa.tem("nome"))              // verdadeiro
escreva(pessoa.obtem("cep", "sem cep"))  // o valor da chave, ou o padrão se não existir
escreva(pessoa.chaves())                 // lista das chaves
escreva(pessoa.valores())                // lista dos valores
pessoa.remova("cidade")                  // remove um par
```

O método `obtem` é útil para evitar erros: em vez de acessar uma chave que talvez não exista, você pede o valor e informa um padrão para o caso de ela faltar.

Para percorrer um dicionário, use `para cada`: a variável recebe cada chave, e você acessa o valor correspondente indexando o dicionário:

```kaju
para cada chave em pessoa {
    escreva(chave, "=>", pessoa[chave])
}
```

## Comparando coleções

O operador `==` compara listas e dicionários pelo **conteúdo**, não pela identidade: duas coleções são iguais quando têm os mesmos elementos (nas mesmas posições, no caso das listas) ou os mesmos pares chave-valor. A comparação é recursiva, então funciona também com coleções aninhadas:

```kaju
escreva([1, 2] == [1, 2])                  // verdadeiro
escreva({"a": 1} == {"a": 1})              // verdadeiro
escreva([1, [2, 3]] == [1, [2, 3]])        // verdadeiro
```

## A função intervalo

Muitas vezes você precisa de uma lista de inteiros em sequência. A função `intervalo(inicio, fim)` gera essa lista, do `inicio` até logo **antes** de `fim`:

```kaju
escreva(intervalo(0, 5))            // [0, 1, 2, 3, 4]
escreva(intervalo(1, 101).soma())   // 5050
```

Como o resultado é uma lista comum, você pode iterá-la com `para cada` ou usar qualquer método de lista sobre ela.

Continue em [Funções](./funcoes.md).

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

Alguns métodos ajudam a reorganizar a estrutura da lista:

```kaju
escreva([[1, 2], [3], [4, 5]].achate())       // [1, 2, 3, 4, 5]  (achata um nível)
escreva([1, 2, 3].combine(["a", "b", "c"]))   // [[1, a], [2, b], [3, c]]  (zíper)
escreva(["x", "y", "z"].enumere())            // [[0, x], [1, y], [2, z]]  (índice + valor)
```

- `achate()` junta as sublistas em uma só, expandindo um nível de profundidade;
- `combine(outra)` casa esta lista com `outra` posição a posição, formando pares `[a, b]` (para no fim da menor);
- `enumere()` devolve cada elemento acompanhado do seu índice, no formato `[indice, valor]` — útil em `para cada`.

### Fatiando com colchetes

Além do método `fatie`, você pode extrair uma sublista com a sintaxe `[inicio:fim]` — do índice `inicio` até logo **antes** de `fim`. Qualquer um dos lados pode ser omitido, e índices **negativos** contam a partir do fim (`-1` é o último):

```kaju
var l = [10, 20, 30, 40, 50]
escreva(l[1:3])    // [20, 30]
escreva(l[2:])     // [30, 40, 50]   (do índice 2 até o fim)
escreva(l[:2])     // [10, 20]       (do começo até antes do 2)
escreva(l[:-1])    // [10, 20, 30, 40]  (tudo menos o último)
escreva(l[-2:])    // [40, 50]       (os dois últimos)
escreva(l[:])      // cópia da lista inteira
```

Limites fora da faixa são ajustados automaticamente, então fatiar nunca dá erro de índice. A mesma sintaxe funciona em textos (veja [Textos](./textos.md)).

### Transformando listas

Alguns métodos recebem uma **função** como argumento e a aplicam a cada elemento. Eles são muito úteis para transformar ou filtrar dados sem escrever laços manualmente:

- `mapeie(f)` cria uma nova lista aplicando `f` a cada elemento;
- `filtre(f)` cria uma nova lista só com os elementos para os quais `f` devolve verdadeiro;
- `reduza(inicial, f)` combina todos os elementos em um único valor, partindo de `inicial`;
- `encontre(f)` devolve o primeiro elemento para o qual `f` é verdadeiro, ou `nulo`;
- `algum(f)` devolve verdadeiro se `f` for verdadeiro para **ao menos um** elemento;
- `todos(f)` devolve verdadeiro se `f` for verdadeiro para **todos** os elementos;
- `agrupe(f)` monta um dicionário agrupando os elementos pela chave devolvida por `f`.

```kaju
var nums = [1, 2, 3, 4, 5]
escreva(nums.mapeie(funcao(x) { retorne x * 2 }))            // [2, 4, 6, 8, 10]
escreva(nums.filtre(funcao(x) { retorne x % 2 == 0 }))       // [2, 4]
escreva(nums.reduza(0, funcao(acc, x) { retorne acc + x }))  // 15
escreva(nums.encontre(funcao(x) { retorne x > 3 }))          // 4
escreva(nums.algum(funcao(x) { retorne x > 4 }))             // verdadeiro
escreva(nums.todos(funcao(x) { retorne x > 0 }))             // verdadeiro
```

O `agrupe` é ótimo para classificar dados: a chave devolvida por `f` (convertida em texto) vira a chave do dicionário, e o valor é a lista dos elementos daquele grupo.

```kaju
var palavras = ["ana", "ari", "bia", "bruno"]
escreva(palavras.agrupe(funcao(p) { retorne p.fatie(0, 1) }))
// {"a": [ana, ari], "b": [bia, bruno]}
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

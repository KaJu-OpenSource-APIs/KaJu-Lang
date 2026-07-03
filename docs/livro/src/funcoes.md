# Funções

Funções organizam o código em blocos com nome que você pode reaproveitar. Você já vem usando uma delas — `escreva` — desde o começo do livro. Neste capítulo vamos definir as nossas.

Uma função é declarada com a palavra-chave `funcao`, seguida do nome, da lista de parâmetros entre parênteses e do corpo entre chaves:

```kaju
funcao soma(a, b) {
    retorne a + b
}

escreva(soma(2, 3))   // 5
```

Aqui `a` e `b` são os **parâmetros** da função: nomes que representam os valores recebidos a cada chamada. Quando escrevemos `soma(2, 3)`, dizemos que `2` e `3` são os **argumentos**, e dentro do corpo `a` vale `2` e `b` vale `3`.

O comando `retorne` encerra a função e devolve um valor a quem a chamou. Uma função pode ter vários `retorne`, e o primeiro que for alcançado interrompe a execução:

```kaju
funcao sinal(n) {
    se n > 0 {
        retorne "positivo"
    }
    se n < 0 {
        retorne "negativo"
    }
    retorne "zero"
}

escreva(sinal(-4))   // negativo
```

## Funções que não devolvem valor

Nem toda função precisa produzir um resultado; algumas existem só pelo efeito que causam, como imprimir na tela:

```kaju
funcao cumprimente(nome) {
    escreva("Olá, " + nome + "!")
}

cumprimente("Ana")   // Olá, Ana!
```

Quando uma função chega ao fim sem encontrar um `retorne`, ela devolve `nulo`. O mesmo acontece se você usar `retorne` sem valor — útil para sair mais cedo de uma função:

```kaju
funcao avise(mensagem) {
    se mensagem == "" {
        retorne          // sai sem fazer nada
    }
    escreva("Aviso: " + mensagem)
}
```

## Parâmetros padrão

Um parâmetro pode ter um valor padrão, usado quando o argumento correspondente não é passado na chamada:

```kaju
funcao saudar(nome, saudacao = "Olá") {
    escreva(saudacao + ", " + nome)
}

saudar("Ana")             // Olá, Ana
saudar("Ana", "Bom dia")  // Bom dia, Ana
```

Parâmetros com valor padrão devem vir depois dos parâmetros sem padrão. Assim, os argumentos que você fornece sempre preenchem os primeiros parâmetros, e os que faltam recorrem aos padrões.

## Argumentos nomeados

Ao chamar uma função, você pode identificar os argumentos pelo nome do parâmetro, na forma `nome: valor`. Isso deixa a chamada mais clara e libera a ordem:

```kaju
funcao criar(nome, idade, cidade) {
    escreva(nome, idade, cidade)
}

criar(nome: "Ana", cidade: "Recife", idade: 30)   // ordem livre
criar("Ana", cidade: "Recife", idade: 30)         // posicionais primeiro
```

Você pode misturar os dois estilos, desde que os **posicionais venham antes** dos nomeados. Argumentos nomeados combinam bem com parâmetros padrão — informe só os que quiser mudar:

```kaju
funcao conectar(host, porta = 8080, tls = falso) { ... }
conectar("exemplo.com", tls: verdadeiro)   // usa a porta padrão
```

As mesmas regras valem para construtores (`novo Classe(x: 1, y: 2)`) e métodos de classes. Passar um nome que não existe, repetir um argumento ou usar nomes em funções embutidas gera um erro claro.

## Parâmetros variádicos

Às vezes você não sabe de antemão quantos argumentos uma função vai receber. Um parâmetro precedido de `...` coleta todos os argumentos restantes em uma lista:

```kaju
funcao total(...numeros) {
    retorne numeros.soma()
}

escreva(total(1, 2, 3, 4))   // 10
escreva(total())             // 0
```

Dentro da função, `numeros` é uma lista comum, que você pode percorrer, medir ou processar como qualquer outra. O parâmetro variádico deve ser sempre o último da lista, e pode conviver com parâmetros normais:

```kaju
funcao registrar(nivel, ...mensagens) {
    para cada m em mensagens {
        escreva("[" + nivel + "] " + m)
    }
}

registrar("info", "iniciado", "carregado", "pronto")
```

## Funções são valores

Em kaju, uma função é um valor como qualquer outro: você pode guardá-la em uma variável, passá-la como argumento e devolvê-la de outra função. Isso é o que chamamos de funções de **primeira classe**.

Guardar uma função em uma variável dá a ela outro nome. A variável passa a ser chamável do mesmo jeito:

```kaju
funcao soma(a, b) {
    retorne a + b
}

var operacao = soma
escreva(operacao(10, 20))   // 30
```

Você também pode criar uma função **anônima** — sem nome — diretamente onde precisa dela. A sintaxe é a mesma, apenas sem o nome:

```kaju
var dobro = funcao(x) { retorne x * 2 }
escreva(dobro(21))          // 42
```

Como funções são valores, elas podem ser passadas para outras funções. Vários métodos de lista tiram proveito disso, recebendo uma função que descreve o que fazer com cada elemento:

```kaju
var numeros = [1, 2, 3, 4]
var quadrados = numeros.mapeie(funcao(n) { retorne n * n })
escreva(quadrados)          // [1, 4, 9, 16]
```

## Closures

Uma função criada dentro de outra lembra o ambiente em que nasceu: ela continua enxergando as variáveis do escopo onde foi definida, mesmo depois que esse escopo termina. Uma função com essa memória é chamada de **closure**.

O exemplo clássico é um contador. `criarContador` define uma variável `total` e devolve uma função que a incrementa. Cada função devolvida guarda seu próprio `total`:

```kaju
funcao criarContador() {
    var total = 0
    retorne funcao() {
        total += 1
        retorne total
    }
}

var proximo = criarContador()
escreva(proximo())   // 1
escreva(proximo())   // 2
escreva(proximo())   // 3
```

A variável `total` não é visível de fora de `criarContador`, mas continua viva enquanto a função devolvida existir. Closures permitem que uma função carregue consigo um pequeno estado privado, o que é a base de muitos padrões úteis.

Agora que você sabe agrupar comportamento em funções, o próximo passo é agrupar dados e comportamento juntos com [orientação a objetos](./objetos.md).

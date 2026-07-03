# Textos

Um **texto** é uma sequência de caracteres escrita entre aspas duplas. É com textos que você guarda nomes, mensagens, linhas de um arquivo — qualquer informação feita de letras, dígitos e símbolos.

```kaju
escreva("Olá, mundo!")
```

Dentro de um texto você pode usar **escapes**: combinações que começam com `\` e representam caracteres especiais. Os mais comuns são `\n` (quebra de linha), `\t` (tabulação), `\"` (uma aspa dupla) e `\\` (uma barra invertida):

```kaju
escreva("primeira linha\nsegunda linha")
```

Esse programa imprime duas linhas, porque `\n` faz o cursor passar para a linha seguinte.

## Juntando textos

O operador `+` **concatena**, ou seja, une dois textos em um só. Quando um dos lados é texto, o outro valor é convertido automaticamente para texto antes de juntar:

```kaju
var nome = "Ana"
escreva("Olá, " + nome + "!")     // Olá, Ana!
escreva("Total: " + 42)           // Total: 42
```

## Interpolação

Concatenar com `+` funciona, mas quando há muitas partes o texto fica difícil de ler. Para esses casos, prefixe o texto com `$`: assim você pode inserir expressões diretamente entre chaves, e cada expressão é calculada e transformada em texto no lugar onde aparece.

```kaju
var nome = "Júlio"
var idade = 30
escreva($"Olá, {nome}! Ano que vem você terá {idade + 1}.")
```

A saída é `Olá, Júlio! Ano que vem você terá 31.`. Observe que a chave aceita qualquer expressão, como o `idade + 1` acima.

Se você precisar de uma chave literal dentro de um texto interpolado, escreva `{{` e `}}`:

```kaju
escreva($"chaves literais: {{ e }}")   // chaves literais: { e }
```

## Métodos de texto

Todo texto oferece **métodos**: operações que você chama com um ponto (`.`) logo após o valor. Um método sempre devolve um resultado — em geral um novo texto — sem alterar o texto original.

Comece pelos métodos que informam algo sobre o texto:

```kaju
escreva("caju".tamanho())          // 4
escreva("caju".contem("aj"))       // verdadeiro
escreva("caju".comecaCom("ca"))    // verdadeiro
escreva("caju".terminaCom("ju"))   // verdadeiro
escreva("caju".indiceDe("j"))      // 2
```

`indiceDe` devolve a posição em que a busca aparece. As posições começam em `0`, então em `"caju"` a letra `c` está na posição 0, `a` na 1, `j` na 2.

Outros métodos produzem um novo texto a partir do original:

```kaju
var s = "  Olá Mundo  "
escreva(s.apara())                  // "Olá Mundo"  (sem espaços nas pontas)
escreva(s.maiusculas())             // "  OLÁ MUNDO  "
escreva(s.minusculas())             // "  olá mundo  "
escreva("caju".fatie(0, 2))         // "ca"  (do índice 0 até antes do 2)
escreva("caju".substitua("u", "!")) // "caj!"
escreva("ab".repita(3))             // "ababab"
```

`fatie(inicio, fim)` recorta um trecho: começa no índice `inicio` e vai até logo **antes** de `fim`. Para recortes rápidos, existe também a sintaxe de fatiamento com colchetes `[inicio:fim]`, com limites opcionais e índices negativos contando a partir do fim:

```kaju
escreva("kajuzinho"[0:4])   // "kaju"
escreva("kajuzinho"[-3:])   // "nho"  (os três últimos)
escreva("kajuzinho"[:-5])   // "kaju" (tudo menos os cinco últimos)
```

Há ainda um par de métodos que conectam textos e listas. `divida` quebra um texto em uma lista, usando um separador; `junte` faz o caminho inverso, unindo os elementos de uma lista com um separador entre eles:

```kaju
escreva("a,b,c".divida(","))        // [a, b, c]
escreva(["a", "b", "c"].junte("+")) // "a+b+c"
```

Como cada método devolve um valor, você pode **encadear** chamadas, aplicando um método sobre o resultado do anterior:

```kaju
escreva("a,b,c".divida(",").junte("+").maiusculas())   // A+B+C
```

## Formatando números como texto

Ao converter um número decimal para texto, às vezes você quer um número fixo de casas — por exemplo, para mostrar valores em reais. A função `formateDecimal(n, casas)` faz isso:

```kaju
escreva(formateDecimal(3.1, 2))              // "3.10"
escreva("R$ " + formateDecimal(1250.5, 2))   // R$ 1250.50
```

Agora vamos ao [Controle de fluxo](./controle.md).

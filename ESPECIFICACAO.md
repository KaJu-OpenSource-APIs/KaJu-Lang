# Especificação da linguagem kaju

> Documento normativo do **kaju** — linguagem de programação de alto nível, **interpretada**, de **uso geral** e com sintaxe e biblioteca **totalmente em português**.
> O interpretador é escrito em **Rust**. O código kaju roda direto no interpretador, sem gerar arquivo intermediário (mesmo modelo do CPython, trocando C por Rust).
> Versão da especificação: **0.1 (rascunho)** · Estilo de sintaxe: **moderno** (chaves `{ }`, enxuto).

## Sumário
1. [Visão geral](#1-visão-geral)
2. [Léxico](#2-léxico)
3. [Tipos e valores](#3-tipos-e-valores)
4. [Expressões e operadores](#4-expressões-e-operadores)
5. [Declarações e comandos](#5-declarações-e-comandos)
6. [Funções](#6-funções)
7. [Classes e objetos](#7-classes-e-objetos)
8. [Exceções (tente/capture)](#8-exceções-tentecapture)
9. [Módulos (importe)](#9-módulos-importe)
10. [Biblioteca padrão (embutidos)](#10-biblioteca-padrão-embutidos)
11. [Tratamento de erros](#11-tratamento-de-erros)
12. [Gramática (EBNF)](#12-gramática-ebnf)
13. [Arquitetura do interpretador (Rust)](#13-arquitetura-do-interpretador-rust)

---

## 1. Visão geral

- **Paradigma:** imperativo/procedural com **funções de primeira classe** e **orientação a objetos** (classes, herança). Tipagem **dinâmica**.
- **Extensão de arquivo:** `.kaju` (ou `.kj`, forma curta)
- **Codificação:** UTF-8 (aceita acentos em identificadores e strings).
- **Ponto de entrada:** o arquivo é executado de cima para baixo; não é obrigatória uma função `principal`.
- **Filosofia:** ler como português claro, ser enxuta e dar **mensagens de erro em português**.

Exemplo mínimo (`ola.kaju`):

```kaju
escreva("Olá, mundo!")
```

Exemplo com mais recursos:

```kaju
funcao saudar(nome) {
    retorne "Olá, " + nome + "!"
}

var idade = 20
se idade >= 18 {
    escreva(saudar("Ana"))
} senao {
    escreva("Menor de idade")
}

var frutas = ["caju", "manga", "acerola"]
para cada fruta em frutas {
    escreva(fruta)
}
```

---

## 2. Léxico

### 2.1 Comentários
```kaju
// comentário de uma linha
/* comentário
   de várias linhas */
```

### 2.2 Identificadores
Começam por letra (incluindo acentuadas) ou `_`, seguidos de letras, dígitos ou `_`.
Válidos: `nome`, `total_geral`, `número`, `_temp`. Diferenciam maiúsculas de minúsculas.

### 2.3 Palavras-chave reservadas
São 40 palavras reservadas:
```
var        constante   funcao      retorne
se         senao       senaose     escolha
caso       padrao      enquanto    para
cada       em          de          ate
passo      pare        continue    e
ou         nao         verdadeiro  falso
nulo       classe      registro    herda
metodo     construtor  novo        isto
base       estatico    tente       capture
finalmente lance       importe     como
```

### 2.4 Literais
- **Número:** `42`, `3.14`, `-7`, `1000` (ver §3).
- **Texto:** `"entre aspas duplas"`, com escapes `\n`, `\t`, `\\`, `\"`.
- **Texto interpolado:** `$"Olá, {nome}! Você tem {idade} anos"` — o prefixo `$` habilita interpolação; cada `{expressao}` dentro das aspas é avaliada e concatenada como texto (ver §4.7).
- **Lógico:** `verdadeiro`, `falso`.
- **Nulo:** `nulo`.
- **Lista:** `[1, 2, 3]`.
- **Dicionário:** `{"chave": valor, "outra": 2}`.

### 2.5 Símbolos
```
+  -  *  /  %              (aritméticos)
& | ^ ~ << >>             (bits: e, ou, xor, não, desloca esq./dir.)
== != < > <= >=           (comparação)
=                         (atribuição)
+= -= *= /= %=            (atribuição composta)
? :                       (operador condicional / ternário)
??                        (coalescência de nulo)
?.                        (acesso opcional / encadeamento seguro)
|>                        (encadeamento / pipe)
( ) { } [ ]              (agrupamento/blocos/coleções)
,  :  .                   (separadores/acesso)
...                       (parâmetro variádico e espalhamento)
$"..."                    (prefixo de texto interpolado, com {} internos)
```

---

## 3. Tipos e valores

| Tipo kaju | Descrição | Exemplo |
|-----------|-----------|---------|
| `numero`  | Número inteiro (i64) ou decimal (f64) — ver nota abaixo | `42`, `3.14` |
| `texto`   | Cadeia de caracteres UTF-8 | `"caju"` |
| `logico`  | Verdadeiro ou falso | `verdadeiro` |
| `lista`   | Sequência ordenada e mutável | `[1, "dois", verdadeiro]` |
| `dicionario` | Mapa chave→valor (chaves texto) | `{"a": 1}` |
| `funcao`  | Função (valor de primeira classe) | `funcao(x){ retorne x }` |
| `classe`  | Molde de objetos (§7) | `classe Gato { ... }` |
| `objeto`  | Instância de uma classe (§7) | `novo Gato("Félix")` |
| `nulo`    | Ausência de valor | `nulo` |

> `tipo(x)` retorna `"objeto"` para instâncias; para descobrir a classe use `classeDe(x)` (retorna o nome da classe como `texto`).

**Modelo numérico (inteiro/decimal sob um só `numero`, à la Lua 5.3):** existe um único tipo visível `numero` (`tipo(x)` sempre retorna `"numero"`), mas internamente um número é **inteiro** (i64) ou **decimal** (f64):

- Literais sem ponto são inteiros (`5`); com ponto são decimais (`5.0`, `3.14`).
- `+ - *` entre inteiros dão inteiro (com valor exato, inclusive grandes); se qualquer lado é decimal, o resultado é decimal (`5 + 2.5` → `7.5`).
- `/` (divisão) **sempre** produz decimal, mesmo entre inteiros (`10 / 2` → `5.0`).
- `%` (resto) dá inteiro entre inteiros, decimal caso contrário.
- Comparações são matemáticas: `5 == 5.0` → `verdadeiro`.
- Ao imprimir, decimais mostram o ponto (`5.0`) para se distinguirem de inteiros (`5`).
- `piso`, `teto`, `arredonde` retornam inteiro; `raiz` e `aleatorio` retornam decimal; `potencia` retorna inteiro quando base e expoente são inteiros (expoente ≥ 0), senão decimal; `absoluto` preserva o tipo.
- Em caso de estouro de i64 em `+ - *` entre inteiros (resultado fora do alcance de -9223372036854775808 a 9223372036854775807), a operação falha com o erro `K222`, em vez de virar decimal silenciosamente.

> Divisão inteira não usa `//` (que é comentário); use `piso(a / b)`.

**Veracidade (para condições):** são "falsos" apenas `falso` e `nulo`; todo o resto é "verdadeiro" (inclusive `0` e `""`). *(Decisão a revisar — outra opção é `0`/`""` também serem falsos.)*

---

## 4. Expressões e operadores

### 4.1 Aritméticos
`+` `-` `*` `/` `%` — sobre `numero`. O `+` também **concatena** `texto`.

**Coerção do `+`:** quando **qualquer** dos lados é `texto`, o `+` concatena e o outro valor é convertido para texto automaticamente (mesma conversão de `paraTexto`):
```kaju
"a" + "b"        // "ab"
"Total: " + 42   // "Total: 42"
10 + " itens"    // "10 itens"
```
Se nenhum lado é `texto`, o `+` é soma numérica. Os demais operadores aritméticos (`-` `*` `/` `%`) só operam entre `numero` (ver §3 para o modelo inteiro/decimal).

### 4.2 Comparação
`==` `!=` `<` `>` `<=` `>=` — retornam `logico`.

**Igualdade de coleções e objetos.** `==` e `!=` entre **listas** e entre **dicionários** comparam por **conteúdo** (igualdade estrutural, recursiva): `[1, 2] == [1, 2]` é `verdadeiro` e `{"a": 1} == {"a": 1}` é `verdadeiro`, ainda que sejam coleções distintas na memória. Já os **objetos** são comparados por **identidade** (mesma instância) por padrão — a menos que a classe defina o método especial `igual(outro)` (ver §7.5), caso em que `==` delega a ele.

### 4.3 Lógicos (em português)
`e` (E lógico), `ou` (OU lógico), `nao` (negação). Com **curto-circuito**.
```kaju
se idade >= 18 e temCarteira {
    escreva("pode dirigir")
}
```

### 4.4 Operadores de bits
Operam **apenas entre inteiros** (usar decimais é erro `K012`):

| Operador | Nome |
|----------|------|
| `&` | E bit a bit |
| `\|` | OU bit a bit |
| `^` | OU exclusivo (xor) |
| `~` | NÃO bit a bit (unário) |
| `<<` | desloca à esquerda |
| `>>` | desloca à direita |

O deslocamento não aceita quantidade negativa (erro `K012`); a quantidade é tomada módulo 64.

### 4.5 Atribuição e atribuição composta
`=` atribui a uma variável, índice (`a[i] = ...`) ou campo (`obj.x = ...`). As formas compostas `+= -= *= /= %=` reescrevem `alvo OP= valor` como `alvo = alvo OP valor`:
```kaju
contador += 1      // contador = contador + 1
```

### 4.6 Operador condicional (ternário)
`condicao ? valorSeVerdadeiro : valorSeFalso` — avalia a condição e devolve um dos dois ramos:
```kaju
var rotulo = idade >= 18 ? "adulto" : "menor"
```

### 4.7 Operadores de nulo (`??` e `?.`)
`a ?? b` (**coalescência de nulo**) devolve `a` se `a` não for `nulo`; caso contrário, avalia e devolve `b`. Reage **apenas** a `nulo` (ao contrário de `ou`): `0`, `""` e `falso` passam intactos.

`a?.membro` e `a?.metodo(...)` (**acesso opcional**) só acessam o membro/chamam o método se `a` não for `nulo`; se for, o resultado é `nulo`, sem erro e sem avaliar os argumentos da chamada. Cada `?.` protege o seu próprio operando à esquerda, de modo que o encadeamento (`a?.b?.c`) propaga o `nulo`. O acesso não opcional (`.`) sobre `nulo` continua sendo erro.
```kaju
var cidade = usuario?.endereco?.cidade ?? "desconhecida"
```

### 4.7.1 Encadeamento (`|>`)
`esq |> dir` passa `esq` como **primeiro argumento** da chamada `dir`. Se o alvo de `dir` é um nome que resolve para uma função em escopo, chama-a: `x |> f(a)` ≡ `f(x, a)`. Caso o nome não seja uma função em escopo, a chamada é interpretada como **método**: `x |> maiusculas` ≡ `x.maiusculas()`. Isso permite encadear tanto funções livres quanto os métodos de coleção.
```kaju
usuarios
  |> filtre(funcao(u) { retorne u.ativo })
  |> mapeie(funcao(u) { retorne u.nome })
  |> junte(", ")
```

### 4.8 Interpolação de texto
Um literal `$"..."` avalia cada `{expressao}` interna e concatena tudo como texto (equivale a somar as partes com `+`, sempre em contexto de texto):
```kaju
var nome = "Ana"
escreva($"Olá, {nome}! Daqui a um ano você terá {idade + 1}.")
```

### 4.9 Precedência (da maior para a menor)
Segue exatamente a cadeia do analisador sintático:

1. `()` agrupamento, chamada `f(...)`, indexação `a[i]`, acesso `.` e `?.`
2. `nao`, `-` unário, `~` (não bit a bit)
3. `*` `/` `%`
4. `+` `-`
5. `<<` `>>` (deslocamento)
6. `<` `>` `<=` `>=`
7. `==` `!=`
8. `&` (e bit a bit)
9. `^` (xor bit a bit)
10. `|` (ou bit a bit)
11. `e`
12. `ou`
13. `|>` (encadeamento / pipe)
14. `??` (coalescência de nulo)
15. `? :` (ternário)
16. `=` `+=` `-=` `*=` `/=` `%=` (atribuição, associativa à direita)

### 4.10 Indexação e fatiamento
`alvo[i]` acessa o elemento no índice `i` (lista/texto) ou a chave `i` (dicionário). Índices de lista/texto são inteiros não negativos, contados a partir de `0`.

`alvo[inicio:fim]` **fatia** uma lista ou texto, devolvendo os elementos de `inicio` até logo **antes** de `fim`. Ambos os limites são opcionais (`[inicio:]`, `[:fim]`, `[:]`) e podem ser **negativos**, contando a partir do fim (`-1` é o último). Limites fora da faixa são ajustados ao tamanho, então o fatiamento nunca gera erro de índice. O resultado é sempre uma nova lista/texto.
```kaju
[10, 20, 30, 40][1:3]   // [20, 30]
[10, 20, 30, 40][:-1]   // [10, 20, 30]
"kaju"[-2:]             // "ju"
```

---

## 5. Declarações e comandos

### 5.1 Variáveis
```kaju
var contador = 0        // mutável
constante PI = 3.14159  // imutável (reatribuir gera erro)
contador = contador + 1
contador += 1           // atribuição composta (+= -= *= /= %=)

// desempacotamento
var a, b = 1, 2         // paralelo
a, b = b, a             // troca
var x, y, z = [10, 20, 30]   // a partir de uma lista
```

### 5.2 Condicional
A forma canônica do "senão se" é a palavra única **`senaose`**. Por compatibilidade, o parser também aceita o legado **`senao se`** (dois tokens) com o mesmo significado.
```kaju
se nota >= 7 {
    escreva("aprovado")
} senaose nota >= 5 {         // 'senao se' (duas palavras) também é aceito
    escreva("recuperação")
} senao {
    escreva("reprovado")
}
```

### 5.3 Laços
```kaju
// enquanto (while)
var i = 0
enquanto i < 3 {
    escreva(i)
    i = i + 1
}

// para numérico (de..ate, ate inclusivo)
para i de 1 ate 5 {
    escreva(i)          // 1,2,3,4,5
}

// passo opcional (padrão 1); pode ser negativo para contagem regressiva
para i de 0 ate 10 passo 2 {
    escreva(i)          // 0,2,4,6,8,10
}
para i de 10 ate 1 passo -1 {
    escreva(i)          // 10,9,8,...,1
}

// para cada (for-each) sobre lista ou dicionário
para cada item em [10, 20, 30] {
    escreva(item)
}

// controle de fluxo
enquanto verdadeiro {
    se acabou { pare }        // break
    se pular  { continue }    // continue
}

// escolha (switch/match, sem fall-through)
escolha dia {
    caso 1 { escreva("domingo") }
    caso 2, 3, 4, 5, 6 { escreva("dia útil") }
    caso 7 { escreva("sábado") }
    padrao { escreva("inválido") }
}

// escolha com padrões, desestruturação e guardas
escolha ponto {
    caso [0, 0] { escreva("origem") }
    caso [x, 0] { escreva("eixo X") }
    caso {"tipo": "circulo", "raio": r} se r > 10 { escreva("círculo grande") }
    padrao { escreva("outro") }
}

// operador condicional (ternário)
var rotulo = idade >= 18 ? "adulto" : "menor"
```

No `para` numérico, `passo` define o incremento a cada volta: é opcional (padrão `1`) e pode ser negativo para contar de trás para frente (`para i de 10 ate 1 passo -1`). O passo **zero** é erro `K205`, pois o laço nunca terminaria.

Um `caso` aceita **padrões**, não só valores: um literal casa por igualdade; um nome casa com qualquer valor e o **vincula**; `_` é curinga; `[p, ...resto]` desestrutura listas; `{"chave": p}` desestrutura dicionários. Cada `caso` pode ter uma **guarda** `se condicao`, e o ramo só é escolhido quando o padrão casa **e** a guarda é verdadeira. Como um nome vincula (em vez de comparar), para comparar contra um valor calculado use um literal, uma guarda ou o `padrao`.

---

## 6. Funções

```kaju
funcao soma(a, b) {
    retorne a + b
}

// funções são valores de primeira classe
var operacao = soma
escreva(operacao(2, 3))   // 5

// função anônima
var dobro = funcao(x) { retorne x * 2 }
```

- `retorne` sem valor devolve `nulo`. Função que termina sem `retorne` também devolve `nulo`.
- Escopo **léxico** com closures (a função captura o ambiente onde foi definida).
- Argumentos passados por valor (coleções são referências compartilhadas).

### 6.1 Parâmetros padrão e variádicos

```kaju
// valor padrão: usado quando o argumento não é passado
funcao saudar(nome, saudacao = "Olá") {
    escreva(saudacao + ", " + nome)
}
saudar("Ana")            // Olá, Ana
saudar("Ana", "Oi")      // Oi, Ana

// variádico: '...' coleta o resto dos argumentos em uma lista
funcao soma(...numeros) {
    var total = 0
    para cada n em numeros { total += n }
    retorne total
}
soma(1, 2, 3)            // 6
```

- Parâmetros sem padrão não podem vir depois de um com padrão.
- O parâmetro variádico (`...`) deve ser o último.

### 6.2 Argumentos nomeados
Na chamada, um argumento pode ser identificado pelo nome do parâmetro: `f(nome: valor)`. Os nomeados vêm sempre **depois** dos posicionais (senão `K023`) e podem estar em qualquer ordem entre si. Também valem para construtores (`novo Classe(x: 1)`) e métodos definidos em kaju; funções embutidas e métodos de coleção só aceitam argumentos posicionais (`K226`).
```kaju
funcao conectar(host, porta = 8080, tls = falso) { ... }
conectar("exemplo.com", tls: verdadeiro)   // porta usa o padrão
```
- Passar um nome que não é parâmetro da função gera `K224`; o parâmetro variádico não pode ser passado por nome.
- Preencher o mesmo parâmetro por posição e por nome (ou repetir o nome) gera `K225`.

### 6.3 Espalhamento (`...`)
O operador `...` (o mesmo símbolo do parâmetro variádico) **espalha** uma coleção:
- em uma **chamada**, `f(...lista)` passa cada elemento da lista como um argumento posicional (combina com o variádico do outro lado);
- em um **literal de lista**, `[...a, ...b]` concatena os elementos;
- em um **literal de dicionário**, `{...a, ...b}` mescla os pares (chaves posteriores vencem).

Em listas e argumentos, `...` espera uma `lista`; em dicionários, um `dicionario` — caso contrário, `K227`.
```kaju
funcao soma3(x, y, z) { retorne x + y + z }
soma3(...[1, 2, 3])              // 6
var todos = [...listaA, ...listaB]
var config = {...padrao, "modo": "escuro"}
```

---

## 7. Classes e objetos

kaju tem orientação a objetos com sintaxe enxuta e palavras-chave em português.

### 7.1 Definição
```kaju
classe Animal {
    // construtor: roda ao criar o objeto com 'novo'
    construtor(nome) {
        isto.nome = nome     // 'isto' é a instância atual (self/this)
    }

    metodo falar() {
        escreva(isto.nome + " faz um som")
    }
}
```

- `construtor` é opcional; se ausente, `novo Classe()` cria um objeto sem atributos iniciais.
- **Atributos** não são declarados à parte: nascem ao serem atribuídos em `isto.x = ...`.
- **Métodos** são declarados com `metodo`. Dentro deles, `isto` referencia a instância.

### 7.2 Instanciação e uso
```kaju
var bicho = novo Animal("Rex")
bicho.falar()             // Rex faz um som
escreva(bicho.nome)       // Rex
```

### 7.3 Herança
```kaju
classe Gato herda Animal {
    metodo falar() {
        base.falar()                       // chama o método da superclasse
        escreva(isto.nome + " faz miau")
    }
}

var felix = novo Gato("Félix")
felix.falar()
// Félix faz um som
// Félix faz miau
```

- `herda` estabelece a superclasse; `base` acessa membros da superclasse (equivale a `super`).
- Métodos da subclasse **sobrescrevem** os de mesmo nome da superclasse.
- `construtor` da subclasse deve chamar `base.construtor(...)` se quiser reaproveitar a inicialização do pai.
- Herança **simples** (uma superclasse por classe) na v1.

### 7.4 Membros estáticos
Um membro marcado com `estatico` pertence à **classe**, não às instâncias. Há duas formas:

- **campo estático:** `estatico nome = valor` — um valor inicial compartilhado.
- **método estático:** `estatico metodo nome(...) { ... }` — chamado sem instância.

Ambos são acessados pela própria classe, com `Classe.membro`:
```kaju
classe Contador {
    estatico total = 0
    estatico metodo criar() {
        Contador.total = Contador.total + 1
        retorne novo Contador()
    }
}

Contador.criar()
escreva(Contador.total)   // 1
```
- `Classe.campo` lê (ou, do lado esquerdo de `=`, escreve) um campo estático.
- `Classe.metodo(...)` invoca um método estático; acessar um método estático sem `()` é erro (`K211`). Membro estático inexistente é `K213`.

### 7.5 Métodos especiais (`paraTexto` e `igual`)
Uma classe pode definir métodos com nomes convencionados que o interpretador chama automaticamente em certos contextos:

- **`paraTexto()`** — sem argumentos, deve retornar `texto`. Define como o objeto é convertido para texto: passa a valer em `escreva(obj)`, na concatenação (`"x: " + obj`), na interpolação (`$"{obj}"`) e na impressão do objeto dentro de listas e dicionários. Sem `paraTexto`, o objeto aparece como `<objeto NomeDaClasse>`.
- **`igual(outro)`** — recebe outro valor e deve retornar `logico`. Quando definido, o operador `==` (e `!=`) entre objetos daquela classe chama esse método em vez de comparar por identidade (ver §4.2).

```kaju
classe Ponto {
    construtor(x, y) {
        isto.x = x
        isto.y = y
    }
    metodo paraTexto() {
        retorne $"({isto.x}, {isto.y})"
    }
    metodo igual(outro) {
        retorne isto.x == outro.x e isto.y == outro.y
    }
}

var p = novo Ponto(1, 2)
escreva(p)                         // (1, 2)
escreva(novo Ponto(1, 2) == p)     // verdadeiro
```

### 7.6 Registros (`registro`)
Um **registro** é uma classe de dados enxuta: `registro Nome(campo1, campo2, ...)` declara um tipo cujo construtor, igualdade estrutural e `paraTexto` são **gerados automaticamente**.
```kaju
registro Ponto(x, y)

var a = Ponto(1, 2)             // ou 'novo Ponto(1, 2)'
escreva(a)                      // Ponto(1, 2)
escreva(a == Ponto(1, 2))       // verdadeiro (compara os campos)
escreva(a.x)                    // 1
```
- **Construção:** `Ponto(1, 2)` ou `novo Ponto(1, 2)`; os argumentos podem ser posicionais ou nomeados (`Ponto(y: 2, x: 1)`). Todos os campos são obrigatórios; faltar/sobrar campo é `K201`, campo inexistente por nome é `K224`, campo repetido é `K225`.
- **Igualdade:** dois registros são iguais quando são do mesmo tipo e todos os campos são iguais (recursivamente). Vale em `==`, em `lista.contem(...)`, etc.
- **Texto:** `paraTexto` gera `Nome(v1, v2, ...)`.

Um registro não tem métodos próprios; para comportamento, use uma `classe`.

---

## 8. Exceções (tente/capture)

Erros de execução podem ser **capturados** em tempo de execução, sem travar o programa. (Isto é diferente dos diagnósticos do §11, que são erros detectados/relatados pelo interpretador.)

```kaju
tente {
    var x = 10 / (a - a)          // dispara erro de divisão por zero
} capture (erro) {
    escreva("Falhou: " + erro.mensagem)
} finalmente {
    escreva("isto sempre executa")
}
```

- O bloco `capture (erro)` recebe um **objeto de erro** com pelo menos `.mensagem` (texto) e `.codigo` (ex.: `"K020"`).
- `finalmente` é opcional e executa sempre, com ou sem erro.
- **Lançar** um erro próprio:
```kaju
funcao dividir(a, b) {
    se b == 0 {
        lance "não é possível dividir por zero"
    }
    retorne a / b
}
```
- `lance <expressao>`: se a expressão for `texto`, vira um erro com aquela mensagem; também é possível `lance novo MeuErro(...)` usando uma classe (§7).
- Erros não capturados sobem até o topo e são exibidos no formato rico do §11.

---

## 9. Módulos (importe)

Um programa pode ser dividido em vários arquivos `.kaju` e reutilizado.

```kaju
// arquivo: matematica.kaju
funcao quadrado(x) { retorne x * x }
constante PI = 3.14159
```

```kaju
// arquivo: principal.kaju
importe "matematica.kaju"
escreva(quadrado(5))          // 25
escreva(PI)

importe "utilidades.kaju" como u
u.formatar("olá")             // acesso com prefixo do módulo

importe "geometria.kaju" como geo
var p = novo geo.Ponto(1, 2)  // instancia uma classe qualificada pelo módulo
```

- `importe "caminho.kaju"` traz os nomes públicos (funções, constantes, classes) do arquivo para o escopo atual.
- `importe "caminho.kaju" como u` traz tudo sob o prefixo `u.` (evita conflito de nomes).
- Uma classe importada sob um prefixo é instanciada com o **nome qualificado**: `novo u.Classe(args)` (ver §12; o `novo` aceita tanto `IDENT` simples quanto `IDENT { "." IDENT }`).
- O caminho é relativo ao arquivo que importa. Cada módulo é executado **uma única vez** (cache), mesmo que importado várias vezes.
- *(Decisão futura a definir: controle explícito do que é público/privado no módulo.)*

---

## 10. Biblioteca padrão (embutidos)

### 10.1 Funções globais

| Função | Descrição |
|--------|-----------|
| `escreva(...)` | Imprime os argumentos separados por espaço e quebra linha |
| `escrevaSemQuebra(...)` | Imprime sem quebrar a linha (útil para prompts) |
| `leia()` | Lê uma linha da entrada como `texto` |
| `pergunte(texto)` | Mostra `texto` (sem quebra) e lê uma linha da entrada |
| `tamanho(x)` | Comprimento de `texto`, `lista` ou `dicionario` |
| `tipo(x)` | Nome do tipo de `x` como `texto` |
| `classeDe(x)` | Nome da classe de um `objeto` como `texto` |
| `paraTexto(x)` | Converte para `texto` |
| `paraNumero(x)` | Converte `texto`/`logico` para `numero` |
| `paraInteiro(x)` | Converte para inteiro (trunca decimais) |
| `afirme(cond)` / `afirme(cond, msg)` | Falha com erro `K231` se `cond` for falsa; útil para testes |
| `intervalo(inicio, fim)` | Lista de inteiros `[inicio, fim)` |
| `agora()` | Segundos inteiros desde 1970 (tempo Unix) |
| `relogio()` | Milissegundos desde 1970 (para medir durações) |
| `formatarData(seg)` | `"AAAA-MM-DD HH:MM:SS"` em UTC |
| `formateDecimal(n, casas)` | Texto do número com exatamente `casas` casas decimais |
| `paraJSON(x)` | Serializa um valor em texto JSON |
| `deJSON(texto)` | Converte texto JSON em valor kaju |

### 10.2 Matemática

`raiz`, `absoluto`, `potencia(base, exp)`, `piso`, `teto`, `arredonde`, `arredondePara(n, casas)`, `aleatorio()`, `minimo(...)`, `maximo(...)`, `seno`, `cosseno`, `log`, e a constante `PI`.

### 10.3 Arquivos

`leiaArquivo(caminho)`, `escrevaArquivo(caminho, conteudo)`, `existeArquivo(caminho)`.

### 10.4 Métodos (`.`)

- **lista:** `adicione`, `remova`, `tamanho`, `contem`, `inverta`, `junte(sep)`, `indiceDe`, `fatie(inicio, fim)`, `ordene`, `ordenePor(f)`, `soma`, `achate`, `combine(outra)`, `enumere`, `mapeie(f)`, `filtre(f)`, `reduza(inicial, f)`, `encontre(f)`, `algum(f)`, `todos(f)`, `agrupe(f)`
- **texto:** `maiusculas`, `minusculas`, `tamanho`, `contem`, `apara`, `substitua(de, para)`, `divida(sep)`, `fatie(inicio, fim)`, `indiceDe(sub)`, `comecaCom`, `terminaCom`, `repita(n)`
- **dicionario:** `chaves`, `valores`, `tem(chave)`, `obtem(chave, padrao)`, `remova(chave)`, `tamanho`

---

## 11. Tratamento de erros

**Meta:** diagnósticos com a mesma riqueza dos erros do compilador Rust — porém **todos em português**. Um erro nunca é só uma linha seca: ele localiza, explica *por que* aconteceu e, sempre que possível, **sugere a correção**.

### 11.1 Anatomia de um erro
Todo diagnóstico tem os mesmos componentes:

1. **Cabeçalho** — severidade + **código do erro** + mensagem curta.
2. **Localização** — `arquivo:linha:coluna`.
3. **Trecho do código** — a(s) linha(s) envolvida(s) com `^^^^` apontando o intervalo exato.
4. **Rótulo** no `^^^^` dizendo o que há de errado ali.
5. **`nota:`** — contexto/explicação do porquê (opcional).
6. **`ajuda:`** — sugestão concreta de correção, com o código já corrigido quando aplicável (opcional).

Cada código de erro (`Kxxx`) tem uma página de explicação longa, consultável por `kaju explique K001` (à la `rustc --explain`).

### 11.2 Exemplos

**Variável não definida (provável erro de digitação):**
```
erro[K001]: a variável 'idde' não foi definida
  --> programa.kaju:4:13
   |
 4 |     escreva(idde)
   |             ^^^^ não existe nenhuma variável com este nome
   |
ajuda: você quis dizer 'idade', definida na linha 2?
   |
 4 |     escreva(idade)
   |             ~~~~~
```

**Tipos incompatíveis:**
```
erro[K012]: operação '-' não se aplica entre 'texto' e 'numero'
  --> programa.kaju:1:1
   |
 1 | "abc" - 1
   | ^^^^^   ^ isto é um 'numero'
   | |
   | isto é um 'texto'
   |
nota: o operador '-' só funciona entre dois valores do tipo 'numero'.
ajuda: para juntar textos use '+'. Para converter, use 'paraNumero("abc")'.
```

**Erro de sintaxe (bloco faltando):**
```
erro[K005]: esperava '{' para abrir o corpo do 'se'
  --> programa.kaju:3:14
   |
 3 |     se idade >= 18
   |     --            ^ esperava '{' aqui
   |     |
   |     este 'se' precisa de um bloco entre chaves
   |
ajuda: envolva o corpo em chaves:
   |
 3 |     se idade >= 18 {
 4 |         escreva("maior")
 5 |     }
```

**Divisão por zero (execução):**
```
erro[K020]: divisão por zero
  --> programa.kaju:2:9
   |
 2 |     var x = 10 / (a - a)
   |             ^^^^^^^^^^^^ o divisor vale 0 neste ponto
   |
nota: a divisão por zero não é definida em kaju.
```

### 11.3 Categorias e códigos
Cada erro tem um código `Kxxx` organizado em três faixas. Ao todo há **54 códigos** hoje; cada um tem uma página de explicação (`kaju explique Kxxx`), então esta seção descreve as faixas em vez de listar todos.

- **Núcleo — análise e semântica (`K0xx`, K001–K023).** É a faixa mais antiga e mistura:
  - **sintaxe** — construção mal formada (ex.: `K005` `se`/bloco sem `{`, `K004` parênteses/argumentos, `K010` dicionário, `K013` classe, `K014` `novo`, `K015` `tente/capture`, `K019` ternário sem `:`, `K021` `escolha`, `K022` atribuição múltipla, `K023` argumento posicional depois de nomeado);
  - **execução** que nasceu junto do núcleo — `K001` variável não definida, `K012` operação entre tipos incompatíveis (também usada por bits/deslocamento), `K020` divisão por zero.
- **Léxico (`K1xx`, K101–K104).** Caractere inesperado, texto sem fechar aspas, número mal formado, escape/interpolação inválidos.
- **Execução (`K2xx`, K201–K231).** Erros em tempo de execução: `K201` número de argumentos, `K203` tipo de argumento de método, `K205` limites/passo inválidos do laço `para` (inclusive passo zero), `K206` índice fora da lista, `K211` método usado sem `()` (falta chamar), `K212` método inexistente, `K213` membro inexistente (campo/método de objeto ou membro estático de classe), `K222` estouro de inteiro (soma/subtração/multiplicação cujo resultado passa do alcance de i64, entre -9223372036854775808 e 9223372036854775807, em vez de virar decimal silenciosamente), `K224`/`K225` argumentos nomeados inválidos (parâmetro inexistente / informado duas vezes), `K226` argumentos nomeados onde não são aceitos, `K227` espalhamento (`...`) de um valor que não é a coleção esperada, `K231` afirmação falhou (`afirme`), entre outros.

> Cada código tem uma página longa consultável com `kaju explique <codigo>` (ex.: `kaju explique K016`), à la `rustc --explain`. Ao relatar um erro, o interpretador ainda sugere `dica: rode 'kaju explique Kxxx'`.

### 11.4 Como isso é implementado em Rust
- Um `struct Diagnostico { severidade, codigo, mensagem, span: Span, rotulos: Vec<Rotulo>, notas, ajudas }` — separa a **estrutura** do erro da sua **renderização**.
- Cada `Token` e nó da AST carrega um `Span { inicio, fim, linha, coluna }` para o `^^^^` apontar o intervalo exato (não só a coluna inicial).
- Um **renderizador** desenha a moldura (`|`, `-->`, `^^^^`, `~~~~`) — na prática usaremos a crate **`ariadne`** ou **`codespan-reporting`**, que produzem exatamente esse layout (as mesmas ideias por trás dos erros do `rustc`), com cores no terminal.
- Sugestões de "você quis dizer X?" usam **distância de edição (Levenshtein)** contra as variáveis/funções em escopo — igual ao `rustc`.
- Todo texto (mensagem, nota, ajuda) nasce em português; nada de string em inglês vazando.

---

## 12. Gramática (EBNF)

```ebnf
programa      = { declaracao } ;

declaracao    = decl_var | decl_const | decl_funcao
              | decl_classe | decl_registro | decl_importe | comando ;

(* 'var'/'constante' aceitam desempacotamento: vários nomes, vários valores
   (ou uma única lista à direita) *)
decl_var      = "var" nomes "=" valores ;
decl_const    = "constante" nomes "=" valores ;
nomes         = IDENT { "," IDENT } ;
valores       = expressao { "," expressao } ;

decl_funcao   = "funcao" IDENT "(" [ params ] ")" bloco ;
params        = param { "," param } ;
param         = [ "..." ] IDENT [ "=" expressao ] ;   (* variádico e/ou valor padrão *)

decl_registro = "registro" IDENT "(" [ IDENT { "," IDENT } ] ")" ;
decl_classe   = "classe" IDENT [ "herda" IDENT ] "{" { membro } "}" ;
membro        = construtor | metodo | membro_estatico ;
construtor    = "construtor" "(" [ params ] ")" bloco ;
metodo        = "metodo" IDENT "(" [ params ] ")" bloco ;
membro_estatico = "estatico" ( metodo | IDENT "=" expressao ) ;

decl_importe  = "importe" TEXTO [ "como" IDENT ] ;

comando       = cmd_se | cmd_escolha | cmd_enquanto | cmd_para_num
              | cmd_para_cada | cmd_retorne | cmd_tente | cmd_lance
              | "pare" | "continue" | atrib_multi | expressao ;

cmd_se        = ( "se" | "senaose" ) expressao bloco [ senao_parte ] ;
senao_parte   = "senaose" expressao bloco [ senao_parte ]      (* palavra única, encadeia *)
              | "senao" "se" expressao bloco [ senao_parte ]   (* legado: duas palavras *)
              | "senao" bloco ;
cmd_escolha   = "escolha" expressao "{"
                { "caso" padrao { "," padrao } [ "se" expressao ] bloco }
                [ "padrao" bloco ] "}" ;
padrao        = IDENT                                  (* "_" = curinga; outro nome vincula *)
              | unario                                 (* literal: comparação por igualdade *)
              | "[" [ padrao { "," padrao } [ "," "..." IDENT ] ] "]"
              | "{" [ TEXTO ":" padrao { "," TEXTO ":" padrao } ] "}" ;
cmd_enquanto  = "enquanto" expressao bloco ;
cmd_para_num  = "para" IDENT "de" expressao "ate" expressao [ "passo" expressao ] bloco ;
cmd_para_cada = "para" "cada" IDENT "em" expressao bloco ;
cmd_retorne   = "retorne" [ expressao ] ;
cmd_tente     = "tente" bloco "capture" "(" IDENT ")" bloco
                [ "finalmente" bloco ] ;
cmd_lance     = "lance" expressao ;
atrib_multi   = IDENT { "," IDENT } "=" valores ;   (* ex.: a, b = b, a *)

bloco         = "{" { declaracao } "}" ;

(* Precedência crescente, do topo (menor) para a base (maior) — ver §4.9 *)
expressao     = atribuicao ;
atribuicao    = ternario [ ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" ) atribuicao ] ;
ternario      = coalescencia [ "?" ternario ":" ternario ] ;
coalescencia  = pipe { "??" pipe } ;
pipe          = ou_logico { "|>" ou_logico } ;   (* x |> f(a) injeta x como 1º argumento *)
ou_logico     = e_logico { "ou" e_logico } ;
e_logico      = ou_bit { "e" ou_bit } ;
ou_bit        = xor_bit { "|" xor_bit } ;
xor_bit       = e_bit { "^" e_bit } ;
e_bit         = igualdade { "&" igualdade } ;
igualdade     = comparacao { ( "==" | "!=" ) comparacao } ;
comparacao    = deslocamento { ( "<" | ">" | "<=" | ">=" ) deslocamento } ;
deslocamento  = soma { ( "<<" | ">>" ) soma } ;
soma          = produto { ( "+" | "-" ) produto } ;
produto       = unario { ( "*" | "/" | "%" ) unario } ;
unario        = ( "nao" | "-" | "~" ) unario | chamada ;
chamada       = primario { "(" [ args ] ")" | indexa_ou_fatia | ( "." | "?." ) IDENT } ;
indexa_ou_fatia = "[" ( expressao [ ":" [ expressao ] ] | ":" [ expressao ] ) "]" ;
args          = argumento { "," argumento } ;   (* nomeados só depois dos posicionais *)
argumento     = [ IDENT ":" ] expressao | "..." expressao ;  (* nomeado, posicional ou espalhado *)

primario      = NUMERO | TEXTO | TEXTO_INTERP
              | "verdadeiro" | "falso" | "nulo"
              | "isto" | "base" | novo
              | IDENT | "(" expressao ")" | lista | dicionario | funcao_anon ;
novo          = "novo" IDENT { "." IDENT } "(" [ args ] ")" ;  (* nome simples ou qualificado por módulo *)
lista         = "[" [ elem_lista { "," elem_lista } ] "]" ;
elem_lista    = expressao | "..." expressao ;              (* item ou espalhamento *)
dicionario    = "{" [ entrada_dic { "," entrada_dic } ] "}" ;
entrada_dic   = TEXTO ":" expressao | "..." expressao ;    (* par ou espalhamento *)
funcao_anon   = "funcao" "(" [ params ] ")" bloco ;

(* TEXTO_INTERP: $"...{expressao}..." — cada trecho {expressao} é analisado como
   uma expressao e concatenado (como texto) às partes literais vizinhas. *)
```

---

## 13. Arquitetura do interpretador (Rust)

Interpretador **tree-walking** (percorre a AST). Organização em crate único, módulos:

```
kaju/
├── Cargo.toml
└── src/
    ├── main.rs          // CLI: `kaju arquivo.kaju` e REPL
    ├── lexer.rs         // texto  -> Vec<Token>  (com linha/coluna)
    ├── token.rs         // enum Token
    ├── parser.rs        // Vec<Token> -> AST  (descida recursiva)
    ├── ast.rs           // enum Expr, enum Cmd (inclui Classe, Tente, Lance, Importe)
    ├── interpreter.rs   // percorre a AST e executa
    ├── ambiente.rs      // escopos (Rc<RefCell<HashMap<String, Valor>>>)
    ├── valor.rs         // enum Valor (Numero, Texto, Logico, Lista, Dic, Funcao,
    │                    //             Classe, Objeto, Nulo)
    ├── classe.rs        // struct Classe { nome, metodos, superclasse } e Objeto
    ├── modulo.rs        // carregamento/cache de arquivos importados
    ├── embutidos.rs     // funções da stdlib (escreva, leia, tamanho...)
    └── erros.rs         // struct Diagnostico + renderização em português
```

**Escolhas idiomáticas de Rust:**
- `enum Valor` + `match` para o modelo de valores.
- `enum Expr` / `enum Cmd` para a AST.
- `Rc<RefCell<...>>` para ambientes/escopos aninhados e closures (permite compartilhamento e mutação). **Objetos** também são `Rc<RefCell<Objeto>>` (semântica de referência); **classes** são `Rc<Classe>` compartilhadas.
- `Result<Valor, ErroKaju>` + operador `?` para propagar erros com linha/coluna.
- Controle de fluxo (`retorne`, `pare`, `continue`) modelado como variantes de sinal internas (`Sinal::Retorno(Valor)`, `Sinal::Pare`, ...), capturadas pelo laço/função. **`lance`** é o `Sinal::Erro(Valor)`, capturado por um `tente`/`capture`; se ninguém captura, vira `Diagnostico` no topo.
- Testes com `#[test]` por módulo + testes de ponta a ponta rodando programas `.kaju` de exemplo.

**Dependências sugeridas:** `ariadne` ou `codespan-reporting` (erros ricos do §11); opcional `logos` (lexer) e `rustyline` (REPL com histórico).

---

> As direções futuras do kaju ficam no [ROADMAP.md](ROADMAP.md).

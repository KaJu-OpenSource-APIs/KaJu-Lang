# Especificação da linguagem kaju

> Documento normativo da **kaju** — linguagem de programação de alto nível, **interpretada**, de **uso geral** e com sintaxe e biblioteca **totalmente em português**.
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
14. [Roadmap de implementação](#14-roadmap-de-implementação)

---

## 1. Visão geral

- **Paradigma:** imperativo/procedural com **funções de primeira classe** e **orientação a objetos** (classes, herança). Tipagem **dinâmica**.
- **Extensão de arquivo:** `.kaju`
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
```
var        constante   funcao      retorne
se         senao       enquanto    para
cada       em          de          ate
pare       continue    e           ou
nao        verdadeiro  falso       nulo
classe     herda       metodo      construtor
novo       isto        base        tente
capture    finalmente  lance       importe
como
```

### 2.4 Literais
- **Número:** `42`, `3.14`, `-7`, `1000` (ver §3).
- **Texto:** `"entre aspas duplas"`, com escapes `\n`, `\t`, `\\`, `\"`.
- **Lógico:** `verdadeiro`, `falso`.
- **Nulo:** `nulo`.
- **Lista:** `[1, 2, 3]`.
- **Dicionário:** `{"chave": valor, "outra": 2}`.

### 2.5 Símbolos
```
+  -  *  /  %        (aritméticos)
== != < > <= >=      (comparação)
=                    (atribuição)
( ) { } [ ]          (agrupamento/blocos/coleções)
,  :  .              (separadores/acesso)
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
- Em caso de estouro de i64, a operação promove automaticamente para decimal.

> Divisão inteira não usa `//` (que é comentário); use `piso(a / b)`.

**Veracidade (para condições):** são "falsos" apenas `falso` e `nulo`; todo o resto é "verdadeiro" (inclusive `0` e `""`). *(Decisão a revisar — outra opção é `0`/`""` também serem falsos.)*

---

## 4. Expressões e operadores

### 4.1 Aritméticos
`+` `-` `*` `/` `%` — sobre `numero`. O `+` também **concatena** `texto`: `"a" + "b"` → `"ab"`.

### 4.2 Comparação
`==` `!=` `<` `>` `<=` `>=` — retornam `logico`.

### 4.3 Lógicos (em português)
`e` (E lógico), `ou` (OU lógico), `nao` (negação). Com **curto-circuito**.
```kaju
se idade >= 18 e temCarteira {
    escreva("pode dirigir")
}
```

### 4.4 Precedência (da maior para a menor)
1. `()` agrupamento, chamada `f(...)`, indexação `a[i]`, acesso `.`
2. `nao`, `-` unário
3. `*` `/` `%`
4. `+` `-`
5. `<` `>` `<=` `>=`
6. `==` `!=`
7. `e`
8. `ou`
9. `=` atribuição

---

## 5. Declarações e comandos

### 5.1 Variáveis
```kaju
var contador = 0        // mutável
constante PI = 3.14159  // imutável (reatribuir gera erro)
contador = contador + 1
```

### 5.2 Condicional
```kaju
se nota >= 7 {
    escreva("aprovado")
} senao se nota >= 5 {
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

// para cada (for-each) sobre lista ou dicionário
para cada item em [10, 20, 30] {
    escreva(item)
}

// controle de fluxo
enquanto verdadeiro {
    se acabou { pare }        // break
    se pular  { continue }    // continue
}
```

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
```

- `importe "caminho.kaju"` traz os nomes públicos (funções, constantes, classes) do arquivo para o escopo atual.
- `importe "caminho.kaju" como u` traz tudo sob o prefixo `u.` (evita conflito de nomes).
- O caminho é relativo ao arquivo que importa. Cada módulo é executado **uma única vez** (cache), mesmo que importado várias vezes.
- *(Decisão futura a definir: controle explícito do que é público/privado no módulo.)*

---

## 10. Biblioteca padrão (embutidos)

### 10.1 Funções globais

| Função | Descrição |
|--------|-----------|
| `escreva(...)` | Imprime os argumentos separados por espaço e quebra linha |
| `leia()` | Lê uma linha da entrada como `texto` |
| `tamanho(x)` | Comprimento de `texto`, `lista` ou `dicionario` |
| `tipo(x)` | Nome do tipo de `x` como `texto` |
| `classeDe(x)` | Nome da classe de um `objeto` como `texto` |
| `paraTexto(x)` | Converte para `texto` |
| `paraNumero(x)` | Converte `texto`/`logico` para `numero` |
| `paraInteiro(x)` | Converte para inteiro (trunca decimais) |
| `intervalo(inicio, fim)` | Lista de inteiros `[inicio, fim)` |
| `agora()` | Segundos inteiros desde 1970 (tempo Unix) |
| `relogio()` | Milissegundos desde 1970 (para medir durações) |
| `formatarData(seg)` | `"AAAA-MM-DD HH:MM:SS"` em UTC |
| `paraJSON(x)` | Serializa um valor em texto JSON |
| `deJSON(texto)` | Converte texto JSON em valor kaju |

### 10.2 Matemática

`raiz`, `absoluto`, `potencia(base, exp)`, `piso`, `teto`, `arredonde`, `arredondePara(n, casas)`, `aleatorio()`, `minimo(...)`, `maximo(...)`, `seno`, `cosseno`, `log`, e a constante `PI`.

### 10.3 Arquivos

`leiaArquivo(caminho)`, `escrevaArquivo(caminho, conteudo)`, `existeArquivo(caminho)`.

### 10.4 Métodos (`.`)

- **lista:** `adicione`, `remova`, `tamanho`, `contem`, `inverta`, `junte(sep)`, `indiceDe`, `fatie(inicio, fim)`, `ordene`, `ordenePor(f)`, `soma`, `mapeie(f)`, `filtre(f)`, `reduza(inicial, f)`
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
- **Léxico (`K1xx`)** — caractere inesperado, texto sem fechar aspas, número mal formado.
- **Sintaxe (`K0xx`)** — construção mal formada (ex.: `se` sem `{`, vírgula faltando).
- **Execução (`K2xx`)** — variável indefinida, tipos incompatíveis, divisão por zero, índice fora da lista, argumentos a mais/a menos numa chamada.

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
              | decl_classe | decl_importe | comando ;

decl_var      = "var" IDENT "=" expressao ;
decl_const    = "constante" IDENT "=" expressao ;
decl_funcao   = "funcao" IDENT "(" [ params ] ")" bloco ;
params        = IDENT { "," IDENT } ;

decl_classe   = "classe" IDENT [ "herda" IDENT ] "{" { membro } "}" ;
membro        = construtor | metodo ;
construtor    = "construtor" "(" [ params ] ")" bloco ;
metodo        = "metodo" IDENT "(" [ params ] ")" bloco ;

decl_importe  = "importe" TEXTO [ "como" IDENT ] ;

comando       = cmd_se | cmd_enquanto | cmd_para_num | cmd_para_cada
              | cmd_retorne | cmd_tente | cmd_lance
              | "pare" | "continue" | bloco | expressao ;

cmd_se        = "se" expressao bloco
                { "senao" "se" expressao bloco }
                [ "senao" bloco ] ;
cmd_enquanto  = "enquanto" expressao bloco ;
cmd_para_num  = "para" IDENT "de" expressao "ate" expressao bloco ;
cmd_para_cada = "para" "cada" IDENT "em" expressao bloco ;
cmd_retorne   = "retorne" [ expressao ] ;
cmd_tente     = "tente" bloco "capture" "(" IDENT ")" bloco
                [ "finalmente" bloco ] ;
cmd_lance     = "lance" expressao ;

bloco         = "{" { declaracao } "}" ;

expressao     = atribuicao ;
atribuicao    = ( acesso "=" atribuicao ) | ou_logico ;
ou_logico     = e_logico { "ou" e_logico } ;
e_logico      = igualdade { "e" igualdade } ;
igualdade     = comparacao { ( "==" | "!=" ) comparacao } ;
comparacao    = soma { ( "<" | ">" | "<=" | ">=" ) soma } ;
soma          = produto { ( "+" | "-" ) produto } ;
produto       = unario { ( "*" | "/" | "%" ) unario } ;
unario        = ( "nao" | "-" ) unario | novo | acesso ;
novo          = "novo" IDENT "(" [ args ] ")" ;
acesso        = primario { "(" [ args ] ")" | "[" expressao "]" | "." IDENT } ;
args          = expressao { "," expressao } ;

primario      = NUMERO | TEXTO | "verdadeiro" | "falso" | "nulo"
              | "isto" | "base"
              | IDENT | "(" expressao ")" | lista | dicionario | funcao_anon ;
lista         = "[" [ expressao { "," expressao } ] "]" ;
dicionario    = "{" [ TEXTO ":" expressao { "," TEXTO ":" expressao } ] "}" ;
funcao_anon   = "funcao" "(" [ params ] ")" bloco ;
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

## 14. Roadmap de implementação

**Fase 1 — núcleo executável**
Lexer → parser → AST → interpretador para: literais, `var`/`constante`, aritmética, `+` em texto, comparação, `e`/`ou`/`nao`, `se/senao`, `enquanto`, `para`, funções + closures, `escreva`. Entregável: rodar os exemplos básicos. **Erros ricos (§11) desde já.**

**Fase 2 — OOP e robustez**
Classes, herança, `isto`/`base`, `novo` (§7); exceções `tente`/`capture`/`finalmente`/`lance` (§8); módulos `importe` (§9).

**Fase 3 — "todas as funções"**
Listas e dicionários completos com métodos, módulos de stdlib `texto`/`matematica`/`arquivo`, `leia`, conversões, mais embutidos. REPL interativo.

**Fase 4 — desempenho (opcional)**
Compilar a AST para **bytecode** e executar numa VM em Rust (modelo `clox` do *Crafting Interpreters*).

**Documentação (em paralelo, seguindo a pesquisa):**
Este `ESPECIFICACAO.md` é o artefato normativo. Depois: "O Livro da kaju" (tutorial), referência da stdlib gerada com exemplos **testados no CI** (a lição mais forte da pesquisa), e guias how-to. Estrutura Diátaxis.

---

### Próximo passo sugerido
Confirmada a spec, começo a **Fase 1**: crio o projeto Cargo e implemento lexer + parser + interpretador até rodar os exemplos deste documento.

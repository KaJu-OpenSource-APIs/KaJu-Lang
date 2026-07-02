# kaju

Linguagem de programação de alto nível, **interpretada**, de **uso geral** e com sintaxe e biblioteca **totalmente em português**. O interpretador é escrito em **Rust**.

> Especificação completa da linguagem: [`ESPECIFICACAO.md`](ESPECIFICACAO.md)
> Pesquisa que embasou o design da documentação: [`PESQUISA-DOCUMENTACAO.md`](PESQUISA-DOCUMENTACAO.md)

## Estado atual: Fase 2 completa

Já funciona:
- Tipos `numero`, `texto`, `logico`, `lista`, `dicionario`, `funcao`, `nulo`
- `var` / `constante`, aritmética, `+` que concatena texto
- Comparações e operadores lógicos em português (`e`, `ou`, `nao`) com curto-circuito
- `se` / `senao` / `senao se`, `enquanto`, `para X de A ate B`, `para cada X em lista/dicionario`
- `pare`, `continue`, `retorne`
- Funções de primeira classe e **closures**
- **Indexação** `a[i]` (leitura e atribuição) em listas, textos e dicionários
- **Dicionários** `{"chave": valor}`
- **Operador `.` + métodos** com encadeamento:
  - lista: `adicione`, `remova`, `tamanho`, `contem`, `inverta`, `junte`
  - texto: `maiusculas`, `minusculas`, `tamanho`, `contem`, `apara`, `substitua`, `divida`
  - dicionário: `chaves`, `valores`, `tem`, `remova`, `tamanho`
- Embutidos: `escreva`, `leia`, `tamanho`, `tipo`, `paraTexto`, `paraNumero`
- Matemática: `raiz`, `absoluto`, `potencia`, `piso`, `teto`, `arredonde`, `aleatorio` e a constante `PI`
- **Orientação a objetos**: `classe`, `construtor`, `metodo`, `novo`, `isto`, campos, **herança** (`herda`) e chamadas à superclasse (`base.metodo()`, `base.construtor()`); `classeDe(obj)`
- **Exceções**: `tente`/`capture`/`finalmente` e `lance`. O `capture (erro)` recebe um objeto com `.mensagem` e `.codigo`, e pega tanto o que você `lance` quanto os erros de runtime do interpretador (ex.: divisão por zero)
- **Módulos**: `importe "arquivo.kaju"` traz os nomes para o escopo; `importe "arquivo.kaju" como m` cria um namespace `m.nome`. Caminhos relativos ao arquivo, com cache (cada módulo roda uma vez)
- **Erros ricos em português** (estilo Rust): código do erro, trecho com `^^^^`, `nota:` e `ajuda:` com sugestão "você quis dizer X?"

**Fase 2 completa.** A kaju já é uma linguagem de uso geral com OOP, exceções e módulos. Próximo possível (Fase 3, opcional): VM de bytecode para desempenho. Ver §14 da especificação.

## Como usar

```bash
# compilar
cargo build

# executar um arquivo
cargo run -- exemplos/tour.kaju
# ou, após compilar:
./target/debug/kaju exemplos/ola.kaju

# abrir o REPL interativo
cargo run

# rodar os testes (executam programas .kaju reais e conferem a saída)
cargo test
```

## Exemplo

```kaju
funcao saudar(nome) {
    retorne "Olá, " + nome + "!"
}

var frutas = ["caju", "manga", "acerola"]
para cada fruta em frutas {
    escreva(saudar(fruta))
}
```

Veja mais em [`exemplos/`](exemplos/).

## Estrutura do código

```
src/
├── main.rs         CLI e REPL
├── token.rs        tokens e Span (posição no código)
├── lexer.rs        fonte -> tokens
├── ast.rs          árvore sintática (Expr, Cmd)
├── parser.rs       tokens -> AST (descida recursiva)
├── valor.rs        valores em runtime
├── ambiente.rs     escopos aninhados (closures)
├── interpreter.rs  percorre a AST e executa
├── embutidos.rs    biblioteca padrão mínima
└── erros.rs        diagnósticos ricos em português
```

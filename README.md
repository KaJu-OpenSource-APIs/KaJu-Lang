# kaju

Linguagem de programação de alto nível, **interpretada**, de **uso geral** e com sintaxe e biblioteca **totalmente em português**. O interpretador é escrito em **Rust**. Arquivos usam a extensão `.kaju` ou `.kj`.

> Especificação completa da linguagem: [`ESPECIFICACAO.md`](ESPECIFICACAO.md)
> Pesquisa que embasou o design da documentação: [`PESQUISA-DOCUMENTACAO.md`](PESQUISA-DOCUMENTACAO.md)

## Estado atual: Fase 2 completa

Já funciona:
- Tipos `numero`, `texto`, `logico`, `lista`, `dicionario`, `funcao`, `nulo`
  - `numero` distingue internamente **inteiro** (i64, exato) e **decimal** (f64): `2+2` é inteiro exato, `10/2` é `5.0`, inteiros grandes não perdem precisão; `tipo()` sempre diz `"numero"`
- `var` / `constante`, aritmética, `+` que concatena texto
- Comparações e operadores lógicos em português (`e`, `ou`, `nao`) com curto-circuito
- `se` / `senao` / `senao se`, `enquanto`, `para X de A ate B`, `para cada X em lista/dicionario`
- `pare`, `continue`, `retorne`
- Funções de primeira classe e **closures**, com **parâmetros padrão** (`b = 10`) e **variádicos** (`...resto`)
- Açúcar sintático: **atribuição composta** (`+=`, `-=`, `*=`, `/=`, `%=`), **operadores de bits** (`& | ^ ~ << >>`), **interpolação de strings** (`$"olá {nome}"`)
- **Indexação** `a[i]` (leitura e atribuição) em listas, textos e dicionários
- **Dicionários** `{"chave": valor}`
- **Operador `.` + métodos** com encadeamento:
  - lista: `adicione`, `remova`, `tamanho`, `contem`, `inverta`, `junte`, `indiceDe`, `fatie`, `ordene`, `ordenePor`, `soma`, `mapeie`, `filtre`, `reduza`
  - texto: `maiusculas`, `minusculas`, `tamanho`, `contem`, `apara`, `substitua`, `divida`, `fatie`, `indiceDe`, `comecaCom`, `terminaCom`, `repita`
  - dicionário: `chaves`, `valores`, `tem`, `obtem`, `remova`, `tamanho`
- Comparação `< > <= >=` entre textos (ordem alfabética), além de números
- `se` / `senaose` / `senao` (aceita também o legado `senao se`), `escolha`/`caso`/`padrao` (switch), operador ternário `cond ? a : b`
- Embutidos: `escreva`, `escrevaSemQuebra`, `leia`, `pergunte`, `tamanho`, `tipo`, `classeDe`, `paraTexto`, `paraNumero`, `paraInteiro`, `intervalo`, `agora`, `relogio`, `formatarData`
- Diagnóstico de erros: `kaju explique K016` explica um código de erro em detalhe
- JSON: `paraJSON`, `deJSON`
- Matemática: `raiz`, `absoluto`, `potencia`, `piso`, `teto`, `arredonde`, `arredondePara`, `aleatorio`, `minimo`, `maximo`, `seno`, `cosseno`, `log` e a constante `PI`
- Arquivos: `leiaArquivo`, `escrevaArquivo`, `existeArquivo`
- **Orientação a objetos**: `classe`, `construtor`, `metodo`, `novo`, `isto`, campos, **herança** (`herda`), chamadas à superclasse (`base.metodo()`), e **membros estáticos** (`estatico metodo`, `estatico campo`); `classeDe(obj)`
- **Desempacotamento**: `var a, b = 1, 2`, troca `a, b = b, a`, e a partir de lista `var x, y = [1, 2]`
- **Exceções**: `tente`/`capture`/`finalmente` e `lance`. O `capture (erro)` recebe um objeto com `.mensagem` e `.codigo`, e pega tanto o que você `lance` quanto os erros de runtime do interpretador (ex.: divisão por zero)
- **Módulos**: `importe "arquivo.kaju"` traz os nomes para o escopo; `importe "arquivo.kaju" como m` cria um namespace `m.nome`. Caminhos relativos ao arquivo, com cache (cada módulo roda uma vez)
- **Erros ricos em português** (estilo Rust): código do erro, trecho com `^^^^`, `nota:` e `ajuda:` com sugestão "você quis dizer X?"

**Fase 2 completa.** O kaju já é uma linguagem de uso geral com OOP, exceções e módulos. Próximo possível (Fase 3, opcional): VM de bytecode para desempenho. Ver §14 da especificação.

## Instalação

O `kaju` é um executável nativo autossuficiente. Para **usar** o kaju você só precisa do binário no `PATH` — **não precisa ter Rust instalado**. Rust só é necessário para **compilar** o kaju a partir do código-fonte.

### Usar (sem Rust)

Com o binário `kaju` em mãos (o arquivo `target/release/kaju` ou um binário distribuído):

```bash
chmod +x kaju
mv kaju ~/.local/bin/        # sem sudo (garanta que ~/.local/bin está no PATH)
# ou, para todos os usuários:
sudo mv kaju /usr/local/bin/
```

Depois, o comando funciona em qualquer lugar:

```bash
kaju programa.kaju          # executa um arquivo (.kaju ou .kj)
kaju                        # abre o REPL interativo
kaju explique K016          # explica um código de erro (todos os códigos têm explicação)
kaju --ajuda                # lista os comandos disponíveis
```

### Compilar a partir do código-fonte (precisa de [Rust](https://www.rust-lang.org/pt-BR))

```bash
# instala o comando 'kaju' no PATH (~/.cargo/bin)
cargo install --path .

# ou apenas gera o binário standalone (em target/release/kaju) para distribuir
cargo build --release
```

Durante o desenvolvimento, `cargo run -- programa.kaju` recompila e roda a versão mais recente. Rode os testes com `cargo test` (executam programas `.kaju` reais e conferem a saída).

> Tutorial completo em [`docs/livro`](docs/livro) ("O Livro do kaju").

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
├── main.rs         CLI, REPL e comando 'explique'
├── token.rs        tokens e Span (posição no código)
├── lexer.rs        fonte -> tokens
├── ast.rs          árvore sintática (Expr, Cmd)
├── parser.rs       tokens -> AST (descida recursiva)
├── valor.rs        valores em runtime
├── ambiente.rs     escopos aninhados (closures)
├── interpreter.rs  percorre a AST e executa
├── metodos.rs      métodos das coleções (.mapeie, .ordene, ...)
├── embutidos.rs    biblioteca padrão (funções globais)
├── explicacoes.rs  explicações dos erros (kaju explique)
└── erros.rs        diagnósticos ricos em português
```

## Suporte a editor

Há uma extensão de VS Code em [`editores/vscode-kaju`](editores/vscode-kaju) com realce de sintaxe, ícone de arquivo e configuração da linguagem. Veja o README de lá para instalar.

## Licença

[MIT](LICENSE) © 2026 Julio Barbosa. Uso, modificação e distribuição livres, inclusive comercial, mantendo o aviso de copyright.

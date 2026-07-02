# kaju

Linguagem de programação de alto nível, **interpretada**, de **uso geral** e com sintaxe e biblioteca padrão **totalmente em português**. O interpretador é escrito em **Rust**. Arquivos usam a extensão `.kaju` ou `.kj`.

```kaju
funcao saudar(nome) {
    retorne "Olá, " + nome + "!"
}

var frutas = ["caju", "manga", "acerola"]
para cada fruta em frutas {
    escreva(saudar(fruta))
}
```

## Destaques

- **Português de ponta a ponta** — palavras-chave (`se`, `enquanto`, `funcao`, `classe`), operadores lógicos (`e`, `ou`, `nao`) e biblioteca padrão, tudo em pt-BR.
- **Orientação a objetos** — classes, `construtor`, métodos, herança (`herda`), chamadas à superclasse (`base.metodo()`) e membros estáticos.
- **Exceções** — `tente` / `capture` / `finalmente` e `lance`, com objeto de erro (`.mensagem`, `.codigo`).
- **Módulos** — `importe "arquivo.kaju"` ou `importe "arquivo.kaju" como m`, com cache.
- **Um tipo `numero`** que distingue internamente inteiro exato (i64) e decimal (f64), sem expor a diferença.
- **Coleções ricas** — listas e dicionários com métodos encadeáveis (`.mapeie`, `.filtre`, `.ordene`, `.reduza`…).
- **Funções de primeira classe** — closures, parâmetros padrão (`b = 10`) e variádicos (`...resto`).
- **Erros ricos em português**, no estilo do Rust: código do erro, trecho do código com `^^^^`, `nota:` e `ajuda:` com sugestões ("você quis dizer X?").

## Instalação

O `kaju` é um executável nativo autossuficiente. Para **usar** o kaju você só precisa do binário no `PATH` — **não precisa ter Rust instalado**. O Rust só é necessário para **compilar** o kaju a partir do código-fonte.

### Usar (sem Rust)

Com o binário `kaju` em mãos:

```bash
chmod +x kaju
mv kaju ~/.local/bin/        # sem sudo (garanta que ~/.local/bin está no PATH)
# ou, para todos os usuários:
sudo mv kaju /usr/local/bin/
```

### Compilar a partir do código-fonte (precisa de [Rust](https://www.rust-lang.org/pt-BR))

```bash
# instala o comando 'kaju' no PATH (~/.cargo/bin)
cargo install --path .

# ou apenas gera o binário standalone (em target/release/kaju) para distribuir
cargo build --release
```

## Uso

```bash
kaju programa.kaju          # executa um arquivo (.kaju ou .kj)
kaju                        # abre o REPL interativo
kaju explique K016          # explica um código de erro em detalhe
kaju --ajuda                # lista os comandos disponíveis
```

No REPL, digite código e pressione Enter; blocos com `{ }` pedem mais linhas; as setas ↑/↓ navegam no histórico e Ctrl+D sai.

## Documentação

- **[O Livro do kaju](docs/livro)** — tutorial completo, do zero ao avançado.
- **[Especificação da linguagem](ESPECIFICACAO.md)** — referência normativa da sintaxe e semântica.
- **[Exemplos](exemplos/)** — programas prontos para rodar.

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

Há uma extensão de VS Code em [`editores/vscode-kaju`](editores/vscode-kaju) com realce de sintaxe, ícone de arquivo e configuração da linguagem. Para o Linux, [`editores/linux`](editores/linux) registra o tipo MIME e o ícone dos arquivos no gerenciador de arquivos.

## Contribuindo

Contribuições são bem-vindas. Veja o [guia de contribuição](CONTRIBUTING.md) e o [código de conduta](CODE_OF_CONDUCT.md). O histórico de mudanças fica no [CHANGELOG](CHANGELOG.md).

## Licença

[MIT](LICENSE) © 2026 Julio Barbosa. Uso, modificação e distribuição livres, inclusive comercial, mantendo o aviso de copyright.

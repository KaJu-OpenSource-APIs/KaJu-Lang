# kaju

[![CI](https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/actions/workflows/ci.yml/badge.svg)](https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/actions/workflows/ci.yml)

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
- **Erros ricos em português**: código do erro, trecho do código com `^^^^`, `nota:` e `ajuda:` com sugestões ("você quis dizer X?").

## Instalação

O `kaju` é um executável nativo autossuficiente: depois de instalado, **usar** o kaju não exige ter Rust instalado. O Rust é necessário apenas para **compilar** o kaju a partir do código-fonte — que é como se instala hoje, enquanto não há binários prontos publicados.

Em todos os casos, comece clonando o repositório:

```bash
git clone https://github.com/KaJu-OpenSource-APIs/KaJu-Lang.git
cd KaJu-Lang
```

### Linux e macOS

**Opção A — script de instalação (recomendado).** Compila e instala o comando `kaju` em `~/.local/bin`:

```bash
./install.sh
```

Precisa do [Rust](https://www.rust-lang.org/pt-BR) instalado. Para instalar para todos os usuários (em `/usr/local/bin`), use `PREFIX=/usr/local ./install.sh`. Se o script avisar que a pasta não está no `PATH`, ele mostra a linha exata para adicionar ao seu `~/.bashrc` ou `~/.zshrc`.

**Opção B — via cargo.** Instala o comando `kaju` em `~/.cargo/bin` (que o `rustup` já coloca no `PATH`):

```bash
cargo install --path .
```

### Windows

Instale o [Rust](https://www.rust-lang.org/pt-BR) com o `rustup` e, no PowerShell ou no Prompt de Comando, dentro da pasta do projeto:

```powershell
cargo install --path .
```

Isso gera `kaju.exe` em `%USERPROFILE%\.cargo\bin`, pasta que o `rustup` já adiciona ao `PATH`. Depois, `kaju` funciona em qualquer terminal.

> Alternativa: `cargo build --release` gera o binário em `target\release\kaju.exe`, que você pode copiar para qualquer pasta do seu `PATH`. O `install.sh` também roda no Windows via **WSL** ou **Git Bash**.

### Já tenho o binário (sem Rust)

Se alguém te passou um binário `kaju` pronto (ou você o gerou com `cargo build --release`), basta colocá-lo no `PATH`:

```bash
chmod +x kaju
mv kaju ~/.local/bin/            # sem sudo (garanta que ~/.local/bin está no PATH)
# ou, para todos os usuários:
sudo mv kaju /usr/local/bin/
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

[MIT](LICENSE) © 2026 Julio Barbosa e Kauan Turcato. Uso, modificação e distribuição livres, inclusive comercial, mantendo o aviso de copyright.

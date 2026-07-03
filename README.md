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
- **Testes embutidos** — `afirme(...)` e o comando `kaju teste` para rodar as funções `teste*` de um arquivo ou pasta.
- **Erros ricos em português**: código do erro, trecho do código com `^^^^`, `nota:` e `ajuda:` com sugestões ("você quis dizer X?").

## Instalação

O `kaju` é um executável nativo autossuficiente: para **usar** o kaju basta o binário no `PATH` — **não precisa ter Rust instalado**. Há duas formas de instalar: baixar um **binário pronto** (mais simples) ou **compilar a partir do código-fonte** (precisa de Rust).

### Binário pronto (sem Rust) — recomendado

A cada versão, binários pré-compilados ficam na [página de Releases](https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/releases). Não precisa clonar o repositório.

**Linux x86_64 (inclusive servidor/VPS, via linha de comando):**

```bash
curl -L -o kaju.tar.gz https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/releases/latest/download/kaju-linux-x86_64.tar.gz
tar -xzf kaju.tar.gz
sudo install -m 755 kaju /usr/local/bin/kaju    # instala no PATH (com root)
kaju --versao
```

Sem `sudo`? Instale só para o seu usuário: `mkdir -p ~/.local/bin && mv kaju ~/.local/bin/` (garanta que `~/.local/bin` está no `PATH`).

**macOS:** baixe `kaju-macos-arm64.tar.gz` (Apple Silicon) ou `kaju-macos-x86_64.tar.gz` (Intel), extraia e mova o `kaju` para o `PATH` como acima. Como o binário não é assinado, na primeira execução pode ser preciso liberá-lo: `xattr -d com.apple.quarantine kaju`.

**Windows:** baixe `kaju-windows-x86_64.zip`, extraia o `kaju.exe` para uma pasta e adicione essa pasta ao `PATH` (Configurações → variáveis de ambiente).

> Descompactar apenas coloca o binário na pasta atual; o que o torna disponível em qualquer lugar é **movê-lo para um diretório do `PATH`**.

### Compilar a partir do código-fonte (precisa de [Rust](https://www.rust-lang.org/pt-BR))

Comece clonando o repositório:

```bash
git clone https://github.com/KaJu-OpenSource-APIs/KaJu-Lang.git
cd KaJu-Lang
```

**Linux e macOS** — o script de instalação compila e instala o `kaju` em `~/.local/bin`:

```bash
./install.sh          # ou: PREFIX=/usr/local ./install.sh  (para todos os usuários)
```

Alternativa multiplataforma (Linux, macOS e **Windows**), instalando em `~/.cargo/bin`, que o `rustup` já deixa no `PATH`:

```bash
cargo install --path .
```

> No Windows, rode isso no PowerShell dentro da pasta do projeto. O `install.sh` também funciona no Windows via **WSL** ou **Git Bash**.

## Uso

```bash
kaju programa.kaju          # executa um arquivo (.kaju ou .kj)
kaju                        # abre o REPL interativo
kaju explique K016          # explica um código de erro em detalhe
kaju teste testes/          # roda as funções 'teste*' de um arquivo ou pasta
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

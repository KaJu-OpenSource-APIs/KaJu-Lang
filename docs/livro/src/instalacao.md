# Instalação e primeiros passos

O `kaju` é um **executável nativo autossuficiente**: uma vez instalado, *usar* o kaju **não exige ter o Rust instalado**. Por enquanto — enquanto não há binários prontos publicados — a instalação é feita compilando a partir do código-fonte, o que precisa do [Rust](https://www.rust-lang.org/pt-BR). Em todos os casos, comece clonando o repositório:

```bash
git clone https://github.com/KaJu-OpenSource-APIs/KaJu-Lang.git
cd KaJu-Lang
```

## Linux e macOS

O jeito mais simples é o script de instalação, que compila e coloca o comando `kaju` em `~/.local/bin`:

```bash
./install.sh
```

Se preferir, instale via cargo (em `~/.cargo/bin`, que o `rustup` já deixa no `PATH`):

```bash
cargo install --path .
```

Se o `install.sh` avisar que `~/.local/bin` não está no seu `PATH`, ele mostra a linha exata para adicionar ao `~/.bashrc` ou `~/.zshrc`. Para instalar para todos os usuários, use `PREFIX=/usr/local ./install.sh`.

## Windows

Instale o [Rust](https://www.rust-lang.org/pt-BR) com o `rustup` e, no PowerShell dentro da pasta do projeto:

```powershell
cargo install --path .
```

Isso gera `kaju.exe` em `%USERPROFILE%\.cargo\bin`, pasta que o `rustup` já adiciona ao `PATH`. Depois, `kaju` funciona em qualquer terminal. (O `install.sh` também roda no Windows via WSL ou Git Bash.)

## Binário pronto (sem Rust)

A cada versão, binários pré-compilados ficam na [página de Releases](https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/releases). Num servidor ou VPS Linux (x86_64), dá para baixar e instalar direto pela linha de comando:

```bash
curl -L -o kaju.tar.gz https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/releases/latest/download/kaju-linux-x86_64.tar.gz
tar -xzf kaju.tar.gz
sudo install -m 755 kaju /usr/local/bin/kaju
kaju --versao
```

Se você já tem um binário `kaju` em mãos — extraído de um Release, gerado por `cargo build --release`, ou passado por alguém —, o que o torna disponível em qualquer lugar é **movê-lo para um diretório do `PATH`**:

```bash
cp kaju ~/.local/bin/kaju        # sem sudo (garanta que ~/.local/bin está no PATH)
chmod +x ~/.local/bin/kaju
```

O binário depende apenas das bibliotecas do sistema (`libc`), então roda em qualquer máquina compatível.

> Durante o desenvolvimento, `cargo run -- arquivo.kaju` recompila e executa a versão mais recente do código de uma vez só.

## Seu primeiro programa

Crie um arquivo chamado `ola.kaju` (a extensão `.kj` também funciona):

```kaju
escreva("Olá, mundo!")
```

E rode:

```bash
kaju ola.kaju
```

A saída é:

```
Olá, mundo!
```

A função `escreva` imprime seus argumentos e quebra a linha. Um programa kaju é executado de cima para baixo — não é preciso declarar uma função principal para começar.

## O REPL (modo interativo)

Execute `kaju` sem argumentos para abrir o **REPL**, onde você digita código e vê o resultado na hora:

```
$ kaju
kaju — REPL interativo.
kaju> 2 + 2
4
kaju> var nome = "Ana"
kaju> escreva("Olá, " + nome)
Olá, Ana
```

No REPL:

- Uma expressão solta (como `2 + 2`) já mostra o resultado.
- As setas ↑/↓ navegam no histórico, que fica salvo entre sessões.
- Blocos com `{ }` pedem mais linhas (prompt `....>`), então dá para digitar uma função inteira.
- `Ctrl+C` cancela a linha atual; `Ctrl+D` sai.

## Comandos úteis

```bash
kaju meu_programa.kaju      # executa um arquivo
kaju                        # abre o REPL
kaju explique K020          # explica um código de erro em detalhe
kaju --versao               # mostra a versão instalada
kaju --ajuda                # lista os comandos disponíveis
```

Próximo: [Valores e tipos](./tipos.md)

# Instalação e primeiros passos

O `kaju` é um **executável nativo autossuficiente**: para *usar* o kaju basta o binário no `PATH` — **não exige ter o Rust instalado**. Há duas formas de instalar: baixar um **binário pronto** (mais simples, sem clonar nada) ou **compilar a partir do código-fonte** (precisa do Rust).

## Binário pronto (sem Rust)

A cada versão, binários pré-compilados ficam na [página de Releases](https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/releases). Num Linux x86_64 (inclusive servidor ou VPS), dá para baixar e instalar direto pela linha de comando:

```bash
curl -L -o kaju.tar.gz https://github.com/KaJu-OpenSource-APIs/KaJu-Lang/releases/latest/download/kaju-linux-x86_64.tar.gz
tar -xzf kaju.tar.gz
sudo install -m 755 kaju /usr/local/bin/kaju
kaju --versao
```

Para macOS e Windows, baixe o pacote correspondente na página de Releases. Em qualquer sistema, o que torna o `kaju` disponível em toda parte é **mover o binário para um diretório do `PATH`** (extrair apenas não basta):

```bash
cp kaju ~/.local/bin/kaju        # sem sudo (garanta que ~/.local/bin está no PATH)
chmod +x ~/.local/bin/kaju
```

O binário depende apenas das bibliotecas do sistema (`libc`), então roda em qualquer máquina compatível.

## Compilar a partir do código-fonte

Precisa do [Rust](https://www.rust-lang.org/pt-BR). Comece clonando o repositório:

```bash
git clone https://github.com/KaJu-OpenSource-APIs/KaJu-Lang.git
cd KaJu-Lang
```

No **Linux e macOS**, o script de instalação compila e coloca o `kaju` em `~/.local/bin` (ou em `/usr/local/bin` com `PREFIX=/usr/local ./install.sh`):

```bash
./install.sh
```

Em qualquer sistema — inclusive **Windows** (no PowerShell) —, o cargo instala o `kaju` em `~/.cargo/bin`, que o `rustup` já deixa no `PATH`:

```bash
cargo install --path .
```

> Durante o desenvolvimento, `cargo run -- arquivo.kaju` recompila e executa a versão mais recente do código de uma vez só. O `install.sh` também roda no Windows via WSL ou Git Bash.

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

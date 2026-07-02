# Instalação e primeiros passos

O `kaju` é um **executável nativo autossuficiente**. Para *usar* a kaju, você só precisa do binário `kaju`; **não precisa ter o Rust instalado**. O Rust só entra em cena quando alguém quer *compilar* a kaju a partir do código-fonte.

## Para usar a kaju

Pegue o executável `kaju` — o arquivo `target/release/kaju` gerado pela compilação, ou um binário já distribuído — e coloque-o num diretório do seu `PATH`:

```bash
# copia o binário para um lugar no PATH do seu usuário
cp target/release/kaju ~/.local/bin/kaju
chmod +x ~/.local/bin/kaju
```

Pronto: o comando `kaju` passa a funcionar em qualquer diretório, sem mais nada instalado.

```bash
kaju meu_programa.kaju
```

O binário depende apenas das bibliotecas do sistema (`libc`), então roda em qualquer máquina compatível.

## Para compilar a kaju a partir do código-fonte

Se você quer construir a kaju do zero — por exemplo, para contribuir com o interpretador —, aí sim precisa do [Rust](https://www.rust-lang.org/pt-BR). Na pasta do projeto:

```bash
# gera o binário otimizado em target/release/kaju
cargo build --release

# ou instala direto no PATH (~/.cargo/bin)
cargo install --path .
```

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
```

Próximo: [Valores e tipos](./tipos.md)

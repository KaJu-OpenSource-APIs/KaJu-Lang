# Contribuindo com o kaju

Obrigado pelo interesse em contribuir com o kaju! Este guia explica como preparar o ambiente, propor mudanças e manter a qualidade do projeto.

## Ambiente de desenvolvimento

Para trabalhar no interpretador você precisa do [Rust](https://www.rust-lang.org/pt-BR) (edição 2024).

```bash
git clone <url-do-repositorio> kaju
cd kaju
cargo build            # compila
cargo test             # roda a suíte de testes
cargo run -- programa.kaju   # roda um arquivo durante o desenvolvimento
```

> Lembre-se: o Rust só é necessário para **desenvolver** ou **compilar** o kaju. Quem apenas **usa** a linguagem precisa somente do binário.

## Fluxo de contribuição

1. Abra uma *issue* descrevendo o problema ou a proposta antes de mudanças grandes, para alinhar a abordagem.
2. Crie um branch a partir do branch principal.
3. Faça as mudanças em commits pequenos e coerentes.
4. Garanta que `cargo test` passa e que `cargo fmt` foi aplicado.
5. Abra um *pull request* descrevendo o que mudou e por quê.

## Estilo de código

- Rode `cargo fmt` antes de commitar; o código segue o estilo padrão do `rustfmt`.
- Rode `cargo clippy` e resolva os avisos relevantes.
- Comentários e mensagens ao usuário são em **português (pt-BR)**, com acentuação correta — coerente com a proposta da linguagem.
- Mensagens de erro devem seguir o padrão rico do kaju: código do erro, trecho com `^^^^`, `nota:` e `ajuda:`. Todo código de erro novo precisa de uma explicação em `src/explicacoes.rs` (acessível por `kaju explique`).

## Testes

Os testes de integração ficam em `tests/integracao.rs` e rodam programas `.kaju` reais como subprocesso, conferindo a saída. Ao adicionar uma funcionalidade da linguagem, inclua um teste que exercite o comportamento observável.

```bash
cargo test
```

## Escopo da linguagem

O kaju busca paridade com linguagens como Java e Python **no nível da linguagem nativa** (não em bibliotecas ou ecossistema). Propostas de sintaxe e semântica devem ser refletidas na [`ESPECIFICACAO.md`](ESPECIFICACAO.md), que é a fonte de verdade.

## Reportando bugs

Ao abrir uma *issue* de bug, inclua:

- O programa `.kaju` mínimo que reproduz o problema.
- A saída obtida e a saída esperada.
- A versão do kaju (veja com `kaju --versao`) e o sistema operacional.

## Código de conduta

Ao participar do projeto, você concorda em seguir o [Código de Conduta](CODE_OF_CONDUCT.md).

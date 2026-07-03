# Changelog

Todas as mudanças notáveis neste projeto são documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/)
e o projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [Não lançado]

## [0.1.0] - 2026-07-02

Primeira versão do kaju: uma linguagem interpretada, de uso geral, com sintaxe e
biblioteca padrão totalmente em português.

### Adicionado

- **Tipos**: `numero`, `texto`, `logico`, `lista`, `dicionario`, `funcao`, `nulo`.
  O tipo `numero` distingue internamente inteiro exato (i64) e decimal (f64).
- **Variáveis**: `var` e `constante`, com desempacotamento (`var a, b = 1, 2`) e
  troca (`a, b = b, a`).
- **Controle de fluxo**: `se` / `senaose` / `senao`, `enquanto`,
  `para X de A ate B`, `para cada X em ...`, `pare`, `continue`, `retorne`,
  `escolha` / `caso` / `padrao` e operador ternário `cond ? a : b`.
- **Funções** de primeira classe com closures, parâmetros padrão e variádicos.
- **Orientação a objetos**: `classe`, `construtor`, `metodo`, `novo`, `isto`,
  herança (`herda`), chamadas à superclasse (`base.metodo()`) e membros
  estáticos.
- **Exceções**: `tente` / `capture` / `finalmente` e `lance`.
- **Módulos**: `importe "arquivo.kaju"` e `importe "arquivo.kaju" como m`,
  com cache.
- **Operadores**: aritméticos, comparação, lógicos (`e`, `ou`, `nao`) com
  curto-circuito, atribuição composta (`+=`, `-=`, `*=`, `/=`, `%=`), operadores
  de bits (`& | ^ ~ << >>`) e interpolação de strings (`$"olá {nome}"`).
- **Coleções**: indexação `a[i]`, dicionários `{"chave": valor}` e métodos
  encadeáveis para listas, textos e dicionários.
- **Biblioteca padrão**: E/S (`escreva`, `escrevaSemQuebra`, `leia`, `pergunte`),
  conversões, matemática, data/hora, arquivos e JSON (`paraJSON`, `deJSON`).
- **Diagnósticos ricos** em português no estilo do Rust, com código de erro,
  trecho do código, `nota:` e `ajuda:`.
- **CLI**: execução de arquivos `.kaju` / `.kj`, REPL interativo com histórico e
  entrada multilinha, `kaju explique <codigo>` e `kaju --ajuda`.
- **Ferramentas de editor**: extensão de VS Code e registro de tipo MIME/ícone
  no Linux.



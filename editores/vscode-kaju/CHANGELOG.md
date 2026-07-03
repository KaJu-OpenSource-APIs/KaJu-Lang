# Changelog da extensão kaju para VS Code

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/).

## [Não lançado]

### Adicionado

- Realce dos operadores `??` (coalescência de nulo), `?.` (acesso opcional) e
  `|>` (encadeamento).
- Realce das palavras-chave `registro` e `enum`, com snippets `registro` e `enum`.

## [1.0.0]

### Adicionado

- **Snippets** para as construções da linguagem: `funcao`, `funcaoanon`, `var`,
  `constante`, `se`, `sesenao`, `senaose`, `enquanto`, `parade`, `paracada`,
  `escolha`, `classe`, `classeherda`, `construtor`, `metodo`, `paratexto`,
  `igual`, `tente`, `tentefin`, `lance`, `importe`, `importecomo`, `retorne`,
  `escreva`, `afirme` e `teste`.
- Auto-indentação ao abrir blocos (`onEnterRules`), dobra de código por regiões
  (`// #regiao` / `// #fimregiao`), `wordPattern` com acentos e auto-fechamento de
  comentário de bloco.
- Metadados do repositório (`repository`, `homepage`, `bugs`, `keywords`) e a
  categoria "Snippets".
- Scripts `instalar.sh` e `desinstalar.sh` para instalar a extensão a partir do
  repositório (VS Code, VSCodium/OSS e code-server), sem precisar do Marketplace.

## [0.1.0]

### Adicionado

- Realce de sintaxe para arquivos `.kaju`/`.kj`.
- Ícone de arquivo e configuração da linguagem (comentários, pares, indentação).

# Extensão kaju para VS Code

Realce de sintaxe, snippets, ícone de arquivo e configuração da linguagem [kaju](../../README.md).

## O que ela oferece

- **Realce de sintaxe** para `.kaju`: palavras-chave em português, funções, classes, textos, números, comentários e operadores coloridos.
- **Snippets** para as construções da linguagem — digite o prefixo e pressione Tab. Exemplos: `funcao`, `se`, `sesenao`, `enquanto`, `parade`, `paracada`, `classe`, `classeherda`, `metodo`, `tente`, `escolha`, `importe`, `paratexto`, `igual`, e `teste` (cria uma função de teste para o `kaju teste`).
- **Ícone** para arquivos `.kaju`.
- **Configuração da linguagem**: comentários `//` e `/* */`, auto-fechamento de `{}`, `[]`, `()` e `""`, indentação automática ao abrir blocos e dobra de código por regiões (`// #regiao` / `// #fimregiao`).

## Como instalar

Clone ou baixe o repositório e rode o instalador — ele copia a extensão para a
pasta do VS Code (detecta também VSCodium/OSS e code-server) e remove versões
antigas:

```bash
cd editores/vscode-kaju
./instalar.sh
```

Depois, no editor: `Ctrl+Shift+P` → **Developer: Reload Window**. Abra um arquivo
`.kaju` e o realce e os snippets já funcionam.

Para remover: `./desinstalar.sh` (e recarregue o editor).

### Alternativa: modo desenvolvimento

Abra a pasta `editores/vscode-kaju` no VS Code e pressione **F5** — abre uma janela "Extension Development Host" com a extensão carregada.

### Alternativa: empacotar em .vsix

Se tiver o `vsce` (`npm install -g @vscode/vsce`), dá para gerar um pacote instalável:

```bash
cd editores/vscode-kaju
vsce package
code --install-extension kaju-1.1.0.vsix
```

## Sobre o ícone no explorador de arquivos

O ícone declarado aqui aparece na aba do editor e em temas de ícones que respeitam
o ícone da linguagem. Alguns temas de ícones de arquivo (ex.: Material Icon Theme)
sobrescrevem ícones por conta própria; nesses casos o ícone do kaju pode não
aparecer no explorador, mas o realce de sintaxe funciona sempre.

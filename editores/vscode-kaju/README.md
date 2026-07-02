# Extensão kaju para VS Code

Realce de sintaxe, ícone de arquivo e configuração da linguagem [kaju](../../README.md).

## O que ela oferece

- **Realce de sintaxe** para `.kaju`: palavras-chave em português, funções, classes, textos, números, comentários e operadores coloridos.
- **Ícone** para arquivos `.kaju`.
- **Configuração da linguagem**: comentários `//` e `/* */`, auto-fechamento de `{}`, `[]`, `()` e `""`, indentação automática dentro de blocos.

## Como instalar (modo local, sem publicar)

A forma mais simples é copiar esta pasta para o diretório de extensões do VS Code e recarregar:

```bash
# Linux/macOS
cp -r editores/vscode-kaju ~/.vscode/extensions/kaju-0.1.0
```

Depois, no VS Code: `Ctrl+Shift+P` → **Developer: Reload Window**.

Abra qualquer arquivo `.kaju` e o realce será aplicado automaticamente.

### Alternativa: modo desenvolvimento

Abra a pasta `editores/vscode-kaju` no VS Code e pressione **F5** — abre uma janela "Extension Development Host" com a extensão carregada.

### Alternativa: empacotar em .vsix

Se tiver o `vsce` instalado (`npm install -g @vscode/vsce`):

```bash
cd editores/vscode-kaju
vsce package
code --install-extension kaju-0.1.0.vsix
```

## Sobre o ícone no explorador de arquivos

O ícone declarado aqui aparece na aba do editor e em temas de ícones que respeitam
o ícone da linguagem. Alguns temas de ícones de arquivo (ex.: Material Icon Theme)
sobrescrevem ícones por conta própria; nesses casos o ícone do kaju pode não
aparecer no explorador, mas o realce de sintaxe funciona sempre.

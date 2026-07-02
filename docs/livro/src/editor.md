# Suporte a editor

Escrever kaju fica mais agradável com realce de sintaxe: palavras-chave, textos, números e comentários ganham cores, e o editor entende os comentários e o auto-fechamento de chaves e aspas. Esta seção mostra como habilitar isso no VS Code e como associar o ícone do kaju aos arquivos `.kaju` no Linux.

Os arquivos da linguagem usam a extensão `.kaju` (ou a forma curta `.kj`).

## Realce de sintaxe no VS Code

O projeto inclui uma extensão em `editores/vscode-kaju`. Ela oferece:

- **realce de sintaxe** para `.kaju` — palavras-chave em português, funções, classes, textos, números, comentários e operadores;
- **ícone** para os arquivos `.kaju`;
- **configuração da linguagem** — comentários `//` e `/* */`, auto-fechamento de `{}`, `[]`, `()` e `""`, e indentação automática dentro de blocos.

A forma mais simples de instalar é copiar a pasta para o diretório de extensões do VS Code e recarregar a janela:

```bash
# Linux e macOS
cp -r editores/vscode-kaju ~/.vscode/extensions/kaju-0.1.0
```

Depois, dentro do VS Code, abra a paleta de comandos com `Ctrl+Shift+P` e execute **Developer: Reload Window**. A partir daí, qualquer arquivo `.kaju` recebe o realce automaticamente.

### Modo desenvolvimento

Para experimentar a extensão sem instalá-la, abra a pasta `editores/vscode-kaju` no VS Code e pressione **F5**. Isso abre uma janela "Extension Development Host" com a extensão já carregada.

### Empacotar em .vsix

Se você tiver o `vsce` instalado (`npm install -g @vscode/vsce`), pode gerar e instalar um pacote:

```bash
cd editores/vscode-kaju
vsce package
code --install-extension kaju-0.1.0.vsix
```

### Sobre o ícone no explorador

O ícone da extensão aparece na aba do editor e nos temas de ícones que respeitam o ícone da linguagem. Alguns temas de ícones de arquivo (como o Material Icon Theme) definem os próprios ícones e podem sobrescrever o do kaju no explorador. Nesses casos, o ícone pode não aparecer na lista de arquivos, mas o realce de sintaxe continua funcionando normalmente.

## Ícone dos arquivos no Linux

No Linux, você pode registrar o tipo MIME `text/x-kaju` e associar o ícone do caju aos arquivos `.kaju` no gerenciador de arquivos. Isso segue o padrão freedesktop e funciona com Nautilus, Thunar, Nemo, Dolphin, PCManFM e outros.

A instalação é feita apenas para o seu usuário, sem `sudo`. Na pasta `editores/linux`, execute:

```bash
./instalar.sh
```

O script copia a definição do tipo MIME para `~/.local/share/mime` e o ícone para `~/.local/share/icons`, e atualiza os caches. Pode ser necessário reiniciar o gerenciador de arquivos — ou fazer logout e login — para o ícone aparecer.

Para conferir se deu certo, use o `gio`, que é o backend usado pelos gerenciadores de arquivos:

```bash
gio info -a standard::content-type algum_arquivo.kaju   # -> text/x-kaju
```

Observação: `xdg-mime query filetype` pode responder `text/plain`, porque ele consulta o `file`/libmagic em vez do banco freedesktop. Isso não é um problema — os gerenciadores de arquivos usam o GLib/`gio`, que respeita o glob `*.kaju`.

Para desfazer o registro, execute `./desinstalar.sh` na mesma pasta.

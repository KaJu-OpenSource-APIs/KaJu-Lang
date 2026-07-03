#!/usr/bin/env sh
# Instala a extensão kaju no VS Code (e variantes) copiando-a para a pasta de
# extensões do editor. Não precisa de conta nem de internet — funciona a partir
# do repositório clonado ou baixado.
#
# Uso:
#   ./instalar.sh
#
# Depois, no editor: Ctrl+Shift+P -> "Developer: Reload Window".

set -eu

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

# Lê a versão do package.json (para nomear a pasta kaju-<versao>).
VERSAO=$(sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$DIR/package.json" | head -1)
[ -n "$VERSAO" ] || VERSAO="dev"
NOME="kaju-$VERSAO"

# Pastas de extensões conhecidas: VS Code, VSCode OSS/VSCodium e code-server.
CANDIDATOS="$HOME/.vscode/extensions $HOME/.vscode-oss/extensions $HOME/.vscodium/extensions $HOME/.vscode-server/extensions"

ALVOS=""
for d in $CANDIDATOS; do
    [ -d "$d" ] && ALVOS="$ALVOS $d"
done
# Se nenhuma existir, assume o VS Code padrão.
[ -n "$ALVOS" ] || ALVOS="$HOME/.vscode/extensions"

for base in $ALVOS; do
    mkdir -p "$base"
    # Remove versões antigas da extensão para não duplicar.
    rm -rf "$base"/kaju-*
    destino="$base/$NOME"
    mkdir -p "$destino"
    # Copia apenas os arquivos da extensão (sem os scripts de instalação).
    for item in package.json language-configuration.json README.md CHANGELOG.md LICENSE icons snippets syntaxes; do
        [ -e "$DIR/$item" ] && cp -R "$DIR/$item" "$destino/"
    done
    echo "kaju: instalada em $destino"
done

echo ""
echo "pronto! recarregue o editor: Ctrl+Shift+P -> 'Developer: Reload Window'."
echo "abra um arquivo .kaju e digite, por exemplo, 'classe' + Tab."

#!/usr/bin/env sh
# Remove a extensão kaju do VS Code (e variantes).
#
# Uso:
#   ./desinstalar.sh
#
# Depois, recarregue o editor: Ctrl+Shift+P -> "Developer: Reload Window".

set -eu

CANDIDATOS="$HOME/.vscode/extensions $HOME/.vscode-oss/extensions $HOME/.vscodium/extensions $HOME/.vscode-server/extensions"

removeu=0
for base in $CANDIDATOS; do
    for d in "$base"/kaju-*; do
        [ -d "$d" ] || continue
        rm -rf "$d"
        echo "kaju: removida $d"
        removeu=1
    done
done

if [ "$removeu" -eq 0 ]; then
    echo "kaju: nenhuma extensão kaju encontrada."
fi
echo "recarregue o editor para concluir."

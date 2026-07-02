#!/usr/bin/env bash
# Remove o tipo MIME e o ícone da kaju instalados por instalar.sh.
set -e

rm -f "$HOME/.local/share/mime/packages/kaju.xml"
update-mime-database "$HOME/.local/share/mime" 2>/dev/null || true

for sz in 48x48 128x128 256x256; do
  rm -f "$HOME/.local/share/icons/hicolor/$sz/mimetypes/text-x-kaju.png"
done
gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true

echo "Removido."

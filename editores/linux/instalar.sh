#!/usr/bin/env bash
# Registra o tipo MIME text/x-kaju e o ícone para arquivos .kaju no seu usuário
# (padrão freedesktop — funciona em Nautilus, Thunar, Nemo, PCManFM, Dolphin...).
set -e

DIR="$(cd "$(dirname "$0")" && pwd)"

# 1) Tipo MIME
mkdir -p "$HOME/.local/share/mime/packages"
cp "$DIR/kaju-mime.xml" "$HOME/.local/share/mime/packages/kaju.xml"
update-mime-database "$HOME/.local/share/mime"

# 2) Ícone do tipo MIME (mesmo PNG em alguns tamanhos)
for sz in 48x48 128x128 256x256; do
  destino="$HOME/.local/share/icons/hicolor/$sz/mimetypes"
  mkdir -p "$destino"
  cp "$DIR/text-x-kaju.png" "$destino/text-x-kaju.png"
done
gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true

echo "Pronto! Arquivos .kaju agora têm tipo 'text/x-kaju' e ícone do caju."
echo "Pode ser preciso reiniciar o gerenciador de arquivos para o ícone aparecer."

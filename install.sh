#!/usr/bin/env sh
# Instalador do kaju para Linux e macOS.
#
# Uso:
#   ./install.sh              instala em ~/.local/bin (sem sudo)
#   PREFIX=/usr/local ./install.sh   instala em /usr/local/bin (pode pedir sudo)
#
# O script compila o kaju a partir do código-fonte, então precisa do Rust
# (https://www.rust-lang.org/pt-BR). Para instalar sem Rust, baixe um binário
# pronto e copie-o para uma pasta do seu PATH (veja o README).

set -eu

# Descobre a raiz do projeto (a pasta deste script), para funcionar de qualquer lugar.
RAIZ=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$RAIZ"

# Pasta de destino: ~/.local/bin por padrão; PREFIX=/usr/local usa /usr/local/bin.
if [ "${PREFIX:-}" = "" ]; then
    BIN_DIR="$HOME/.local/bin"
else
    BIN_DIR="$PREFIX/bin"
fi

echo "kaju: instalando a partir de $RAIZ"

# 1. Precisa do cargo (Rust) para compilar.
if ! command -v cargo >/dev/null 2>&1; then
    echo "erro: 'cargo' não encontrado. Instale o Rust em https://www.rust-lang.org/pt-BR" >&2
    echo "      (ou baixe um binário pronto do kaju e copie para o seu PATH)." >&2
    exit 1
fi

# 2. Compila em modo release.
echo "kaju: compilando (cargo build --release)..."
cargo build --release

# 3. Copia o binário para a pasta de destino.
BIN="$RAIZ/target/release/kaju"
if [ ! -f "$BIN" ]; then
    echo "erro: binário não encontrado em $BIN" >&2
    exit 1
fi

mkdir -p "$BIN_DIR"
# Usa sudo automaticamente se a pasta não for gravável (ex.: /usr/local/bin).
if [ -w "$BIN_DIR" ]; then
    cp "$BIN" "$BIN_DIR/kaju"
else
    echo "kaju: $BIN_DIR precisa de permissão de administrador; usando sudo."
    sudo cp "$BIN" "$BIN_DIR/kaju"
fi
chmod +x "$BIN_DIR/kaju" 2>/dev/null || sudo chmod +x "$BIN_DIR/kaju"

echo "kaju: instalado em $BIN_DIR/kaju"

# 4. Avisa se a pasta não está no PATH.
case ":$PATH:" in
    *":$BIN_DIR:"*) : ;;  # já está no PATH
    *)
        echo ""
        echo "atenção: '$BIN_DIR' não está no seu PATH."
        echo "adicione esta linha ao seu ~/.bashrc ou ~/.zshrc:"
        echo "    export PATH=\"$BIN_DIR:\$PATH\""
        echo "depois reabra o terminal (ou rode: source ~/.bashrc)."
        ;;
esac

echo ""
echo "pronto! teste com:  kaju --versao"

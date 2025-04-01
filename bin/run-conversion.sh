#!/bin/bash
# Script para ejecutar el convertidor de formatos Dot ignore

# Verificar si se proporcionaron los argumentos necesarios
if [ $# -lt 3 ]; then
    echo "Error: Se requieren tres argumentos."
    echo "Uso: $0 archivo_entrada archivo_salida formato"
    exit 1
fi

INPUT_FILE="$1"
OUTPUT_FILE="$2"
FORMAT="$3"

# Validar formato
if [[ ! "$FORMAT" =~ ^(git|svn|vcs)$ ]]; then
    echo "Error: El formato debe ser uno de: git, svn, vcs"
    exit 1
fi

# Determinar la ruta del script y la ubicación del binario
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Determinar el sistema operativo para la ruta correcta del binario
case "$(uname -s)" in
    Linux*)     BINARY_PATH="$ROOT_DIR/bin/linux/vcsconvert";;
    Darwin*)    BINARY_PATH="$ROOT_DIR/bin/macos/vcsconvert";;
    CYGWIN*)    BINARY_PATH="$ROOT_DIR/bin/windows/vcsconvert.exe";;
    MINGW*)     BINARY_PATH="$ROOT_DIR/bin/windows/vcsconvert.exe";;
    *)          BINARY_PATH="$ROOT_DIR/bin/linux/vcsconvert";;
esac

# Verificar si el binario existe
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: El binario '$BINARY_PATH' no existe. Por favor, compile el proyecto primero con 'cargo build --release'."
    exit 1
fi

# Verificar si el binario tiene permisos de ejecución
if [ ! -x "$BINARY_PATH" ] && [[ "$BINARY_PATH" != *".exe" ]]; then
    echo "Otorgando permisos de ejecución al binario..."
    chmod +x "$BINARY_PATH"
fi

# Ejecutar el comando de conversión
echo "Convirtiendo $INPUT_FILE a formato $FORMAT..."
"$BINARY_PATH" --input "$INPUT_FILE" --output "$OUTPUT_FILE" --format "$FORMAT"

# Verificar el resultado
if [ $? -eq 0 ]; then
    echo "Conversión completada exitosamente. Archivo guardado en $OUTPUT_FILE"
else
    echo "Error durante la conversión. Código de salida: $?"
    exit $?
fi

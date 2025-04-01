#!/bin/bash
# Script para convertir archivos .gitignore y .svnignore a formato .DotIgnore
# Parte del proyecto DotIgnore

echo "==============================================="
echo "       Dot ignore Format Converter Tool       "
echo "==============================================="

# Verificar argumentos
if [ $# -lt 1 ]; then
    echo "Error: Debe especificar un archivo de origen."
    echo "Uso: convert-ignore [archivo.gitignore|archivo.svnignore] [archivo.DotIgnore]"
    echo
    echo "Ejemplos:"
    echo "  convert-ignore .gitignore"
    echo "  convert-ignore proyecto/.svnignore proyecto/.DotIgnore"
    exit 1
fi

# Determinar argumentos
SOURCE="$1"
DEST="$2"

# Si no se especifica destino, usar .DotIgnore en la misma ubicación
if [ -z "$DEST" ]; then
    DEST="$(dirname "$SOURCE")/.DotIgnore"
fi

# Verificar que el archivo de origen existe
if [ ! -f "$SOURCE" ]; then
    echo "Error: El archivo $SOURCE no existe."
    exit 1
fi

# Ejecutar la conversión
echo
echo "Convirtiendo $SOURCE a formato .DotIgnore..."
vcs convert-ignore -s "$SOURCE" -d "$DEST"

# Verificar si la conversión fue exitosa
if [ $? -ne 0 ]; then
    echo "Error: No se pudo convertir el archivo."
    exit 1
fi

echo
echo "Conversión completada con éxito."
echo "Archivo generado: $DEST"
echo 

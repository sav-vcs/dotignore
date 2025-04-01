# Conversor de Formatos para .DotIgnore

Este documento describe cómo utilizar el conversor de formatos que permite migrar desde archivos `.gitignore` y `.svnignore` al nuevo formato estándar `.DotIgnore`.

## Características

- **Conversión inteligente**: Transforma automáticamente reglas de Git y SVN a formato .DotIgnore.
- **Organización estructurada**: Clasifica automáticamente patrones en grupos semánticos.
- **Identificación de plataformas**: Detecta patrones específicos de sistemas operativos y los organiza en grupos adecuados.
- **Conversión por lotes**: Permite convertir todos los archivos de un repositorio o directorio de forma recursiva.
- **Auto-instalable**: Se instala como una herramienta de línea de comandos en cualquier sistema operativo.
- **Multiplataforma**: Disponible para Windows, macOS y Linux.

## Instalación

### Método Rápido

Ejecuta el siguiente comando para instalar el conversor:

```bash
DotIgnore-converter --install
```

Esto instalará la herramienta en una ubicación apropiada según tu sistema operativo y la hará disponible globalmente.

### Instalación Manual

1. Descarga el archivo `DotIgnore-converter-{version}-<plataforma>.zip` o `.tar.gz` desde la sección de descargas.
2. Extrae el contenido a cualquier directorio.
3. Sigue las instrucciones específicas para tu sistema operativo:

   **Windows:**
   - Agrega el directorio al PATH del sistema, o
   - Crea un acceso directo en una ubicación accesible.

   **macOS/Linux:**
   - Crea un enlace simbólico: `ln -s /ruta/al/DotIgnore-converter /usr/local/bin/`

## Uso

### Convertir un archivo individual

```bash
DotIgnore-converter .gitignore
```

Esto creará un archivo `.DotIgnore` en el mismo directorio, manteniendo el original.

### Especificar ruta de destino

```bash
DotIgnore-converter .gitignore /ruta/destino/.DotIgnore
```

### Convertir un directorio completo (recursivamente)

```bash
DotIgnore-converter /ruta/al/repositorio
```

Esto buscará y convertirá todos los archivos `.gitignore` y `.svnignore` en el directorio y subdirectorios.

### Ver ayuda

```bash
DotIgnore-converter --help
```

## Formato de conversión

El conversor aplica las siguientes transformaciones:

### Desde .gitignore

1. Agrupa los patrones generales en la sección `[patrones_estandar]`
2. Identifica y separa patrones específicos para:
   - Windows: `[windows]`
   - macOS: `[macos]`
   - Linux: `[linux]`
3. Crea un grupo adicional para IDEs/editores: `[editores]`
4. Mantiene todos los comentarios originales

### Desde .svnignore

1. Agrupa los patrones en la sección `[svn_patrones]`
2. Convierte el formato de espacios a líneas individuales
3. Mantiene todos los comentarios originales

## Ejemplos

### Archivo .gitignore original:

```
# Archivos binarios
*.exe
*.dll

# Sistema
.DS_Store
Thumbs.db
```

### Archivo .DotIgnore resultante:

```
# Archivo .DotIgnore convertido desde .gitignore
# Fecha de conversión: 2023-08-15 10:30:45

[patrones_estandar] {
    # Archivos binarios
    *.exe
    *.dll
} # fin patrones_estandar

[macos] {
    .DS_Store
} # fin macos

[windows] {
    Thumbs.db
} # fin windows

[editores] {
    # Archivos específicos de editores/IDEs
    # Comenta o descomenta según sea necesario
    #.idea/
    #.vscode/
    #*.sublime-*
    #*.swp
} # fin editores
```

## Integración con herramientas

### Integración con Git (hooks)

Puedes configurar un hook post-merge para convertir automáticamente los archivos .gitignore:

```bash
#!/bin/sh
# .git/hooks/post-merge

DotIgnore-converter .gitignore
```

### Integración con pipelines de CI/CD

Agrega la conversión a tus flujos de CI/CD:

```yaml
# GitHub Actions
- name: Convertir archivos de ignorar
  run: |
    curl -sSL https://DotIgnore.com/descargar/DotIgnore-converter | bash
    DotIgnore-converter $GITHUB_WORKSPACE
```

## Solución de problemas

### El conversor no reconoce correctamente el formato

Asegúrate de que los archivos tengan los nombres correctos (`.gitignore` o `.svnignore`). Si tienes un archivo con formato Git pero con otro nombre, renómbralo temporalmente para la conversión.

### Error de permisos en Linux/macOS

Si hay problemas de permisos al intentar instalar o ejecutar:

```bash
chmod +x /ruta/al/DotIgnore-converter
sudo DotIgnore-converter --install
``` 

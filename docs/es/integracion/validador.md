# Validador Multiplataforma para el Estándar .DotIgnore

Este documento describe cómo utilizar el validador multiplataforma para archivos `.DotIgnore` conforme al estándar.

## Características

- **Multiplataforma**: Ejecutables independientes para Windows, macOS y Linux.
- **Validación completa**: Verifica la estructura, sintaxis y coherencia de tus archivos `.DotIgnore`.
- **Mensajes claros**: Proporciona mensajes de error y advertencia específicos con sugerencias de corrección.
- **Modo por lotes**: Permite validar todos los archivos `.DotIgnore` en un directorio y sus subdirectorios.
- **Fácil integración**: Puede integrarse en flujos de trabajo de CI/CD y scripts de automatización.

## Instalación

### Windows

1. Descarga el archivo `DotIgnore-validator-{version}-win-x64.zip` desde la sección de descargas.
2. Extrae el contenido a cualquier directorio de tu elección.
3. Opcionalmente, agrega la ruta del directorio al PATH para ejecutar el validador desde cualquier ubicación.

### macOS

1. Descarga el archivo `DotIgnore-validator-{version}-osx-x64.tar.gz` desde la sección de descargas.
2. Extrae el contenido:
   ```bash
   tar -xzf DotIgnore-validator-{version}-osx-x64.tar.gz -C /ruta/destino
   ```
3. Da permisos de ejecución:
   ```bash
   chmod +x /ruta/destino/DotIgnore-validator
   ```
4. Opcionalmente, crea un enlace simbólico:
   ```bash
   ln -s /ruta/destino/DotIgnore-validator /usr/local/bin/DotIgnore-validator
   ```

### Linux

1. Descarga el archivo `DotIgnore-validator-{version}-linux-x64.tar.gz` o `DotIgnore-validator-{version}-linux-arm64.tar.gz` según tu arquitectura desde la sección de descargas.
2. Extrae el contenido:
   ```bash
   tar -xzf DotIgnore-validator-{version}-linux-x64.tar.gz -C /ruta/destino
   ```
3. Da permisos de ejecución:
   ```bash
   chmod +x /ruta/destino/DotIgnore-validator
   ```
4. Opcionalmente, crea un enlace simbólico:
   ```bash
   sudo ln -s /ruta/destino/DotIgnore-validator /usr/local/bin/DotIgnore-validator
   ```

## Uso

### Validar un archivo específico

```bash
DotIgnore-validator /ruta/al/archivo/.DotIgnore
```

### Validar todos los archivos en un directorio

```bash
DotIgnore-validator --batch /ruta/al/directorio
```

### Ver la versión actual

```bash
DotIgnore-validator --version
```

### Mostrar ayuda

```bash
DotIgnore-validator --help
```

## Códigos de salida

- **0**: La validación se completó correctamente sin errores (pueden haber advertencias).
- **1**: Se encontraron errores durante la validación.

## Ejemplos

### Validar un archivo local

```bash
DotIgnore-validator .DotIgnore
```

### Validar un archivo en una ruta específica

```bash
DotIgnore-validator C:\proyectos\mi-repo\.DotIgnore  # Windows
DotIgnore-validator /home/usuario/proyectos/mi-repo/.DotIgnore  # Unix
```

### Validar todos los archivos .DotIgnore en un directorio

```bash
DotIgnore-validator --batch C:\proyectos  # Windows
DotIgnore-validator --batch /home/usuario/proyectos  # Unix
```

## Integración con entornos de desarrollo (IDE)

### Visual Studio Code

Crea un archivo de tarea en `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Validar .DotIgnore",
      "type": "shell",
      "command": "DotIgnore-validator",
      "args": ["${workspaceFolder}/.DotIgnore"],
      "presentation": {
        "reveal": "always",
        "panel": "new"
      },
      "group": {
        "kind": "build",
        "isDefault": true
      }
    }
  ]
}
```

### Integración con Git Hooks

Puedes crear un hook pre-commit para validar el archivo `.DotIgnore` antes de confirmar cambios:

```bash
#!/bin/sh
# .git/hooks/pre-commit

DotIgnore-validator .DotIgnore
if [ $? -ne 0 ]; then
  echo "Error: El archivo .DotIgnore tiene errores. Por favor, corríjalos antes de confirmar."
  exit 1
fi
```

## Solución de problemas

### El ejecutable no funciona en macOS

Si macOS te impide ejecutar el validador debido a que proviene de un "desarrollador no identificado", puedes permitir su ejecución de las siguientes maneras:

1. Desde Finder, haz clic derecho en el ejecutable y selecciona "Abrir".
2. Confirma que deseas abrirlo en el diálogo que aparece.

O bien, desde la terminal:

```bash
xattr -d com.apple.quarantine /ruta/al/DotIgnore-validator
```

### El ejecutable no funciona en Linux

Asegúrate de que tienes los permisos de ejecución correctos:

```bash
chmod +x /ruta/al/DotIgnore-validator
``` 

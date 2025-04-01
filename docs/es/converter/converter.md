# Conversor de Formatos para .ignore

Este documento describe cómo utilizar el conversor de formatos que permite migrar desde archivos `.gitignore` y `.svnignore` al nuevo formato estándar `.ignore` del sistema dotIgnore.

Para documentación completa y actualizada, visite [https://dotignore.dev/docs/converter](https://dotignore.dev/docs/converter).

## Características

- **Conversión de formatos**: Transforma reglas de Git y SVN a formato .ignore
- **Organización por comentarios**: Crea grupos basados en los comentarios del archivo original
- **Mantenimiento de estructura**: Preserva los comentarios y la organización original

## Instalación

### Método Rápido

Ejecuta el siguiente comando para instalar el conversor:

```bash
dotignore convert-ignore --install
```

Esto instalará la herramienta en una ubicación apropiada según tu sistema operativo y la hará disponible globalmente.

### Instalación Manual

1. Descarga el archivo `dotignore-converter-{version}-<plataforma>.zip` o `.tar.gz` desde la sección de descargas.
2. Extrae el contenido a cualquier directorio.
3. Sigue las instrucciones específicas para tu sistema operativo:

   **Windows:**
   - Agrega el directorio al PATH del sistema, o
   - Crea un acceso directo en una ubicación accesible.

   **macOS/Linux:**
   - Crea un enlace simbólico: `ln -s /ruta/al/dotignore-converter /usr/local/bin/`

## Uso

### Convertir un archivo individual

```bash
dotignore convert-ignore .gitignore
```

Esto creará un archivo `.ignore` en el mismo directorio, manteniendo el original.

### Especificar ruta de destino

```bash
dotignore convert-ignore .gitignore /ruta/destino/.ignore
```

### Ver ayuda

```bash
dotignore convert-ignore --help
```

## Formato de conversión

El conversor aplica las siguientes transformaciones:

### Desde .gitignore

1. Crea grupos basados en los comentarios encontrados en el archivo
2. Si no hay comentarios, agrupa los patrones en la sección `[default]`
3. Mantiene todos los comentarios originales

### Desde .svnignore

1. Crea grupos basados en los comentarios encontrados en el archivo
2. Si no hay comentarios, agrupa los patrones en la sección `[default]`
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

### Archivo .ignore resultante:

```
# Archivo .ignore convertido desde .gitignore
# Fecha de conversión: 2023-08-15 10:30:45

[archivos_binarios] {
    *.exe
    *.dll
} # fin archivos_binarios

[sistema] {
    .DS_Store
    Thumbs.db
} # fin sistema
```

## Integración con herramientas

### Integración con Git (hooks)

Puedes configurar un hook post-merge para convertir automáticamente los archivos .gitignore:

```bash
#!/bin/sh
# .git/hooks/post-merge

dotignore convert-ignore .gitignore
```

### Integración con pipelines de CI/CD

Agrega la conversión a tus flujos de CI/CD:

```yaml
# GitHub Actions
- name: Convertir archivos de ignorar
  run: |
    curl -sSL https://dotignore.dev/download/converter | bash
    dotignore convert-ignore $GITHUB_WORKSPACE
```

## Compilación desde código fuente

Para compilar el conversor desde el código fuente:

1. Asegúrate de tener instalado .NET SDK 7.0 o superior.
2. Clona el repositorio:
   ```bash
   git clone https://github.com/sav-project/dotignore-converter.git
   ```
3. Compila el proyecto:
   ```bash
   cd dotignore-converter
   dotnet build -c Release
   ```

## Solución de problemas

### El conversor no reconoce correctamente el formato

Asegúrate de que los archivos tengan los nombres correctos (`.gitignore` o `.svnignore`). Si tienes un archivo con formato Git pero con otro nombre, renómbralo temporalmente para la conversión.

## Licencia

El conversor de formatos se distribuye bajo la licencia MIT.

---

Desarrollado con el apoyo de [SAV Project](https://www.sav-project.com) 
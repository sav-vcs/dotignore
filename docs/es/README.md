# DotIgnore

Un formato unificado para archivos de ignore en todos los sistemas de control de versiones

## Descripción

DotIgnore es un formato estandarizado para definir qué archivos deben ser ignorados en sistemas de control de versiones, compatible con Git, SVN y otros. Ofrece un enfoque más organizado y semántico que los formatos tradicionales, con agrupación, organización jerárquica y características avanzadas.

## Características Principales

- **Grupos organizados** para categorizar patrones de ignore
- **Compatibilidad multiplataforma** con conversión simple entre formatos
- **Preservación de directorios vacíos** sin archivos marcadores
- **Filtrado basado en tamaño** para ignorar archivos según su tamaño
- **Sintaxis clara** con mejor legibilidad y mantenibilidad

## Inicio Rápido

1. **Instalar DotIgnore**:

   ```bash
   cargo install ignore
   ```

2. **Convertir tus archivos ignore existentes**:

   ```bash
   ignore -i .gitignore -o .ignore
   ```

3. **O crear un nuevo archivo DotIgnore**:

   ```bash
   ignore -n -o .ignore
   ```

## Formato

# Ejemplo de formato DotIgnore

```
# Este es un ejemplo de archivo .ignore
# Los patrones fuera de grupos son globales

*.tmp
*.cache

[sistema] {
    # Archivos del sistema
    .DS_Store
    Thumbs.db
    desktop.ini
}

[compilacion] {
    # Artefactos de compilación
    build/
    dist/
    *.o
    *.obj
}

[logs:aplicacion] {
    # Logs de aplicación
    logs/*.log
    !logs/importante.log
}

[tamaño:grande] {
    # Archivos grandes
    size:>50MB *.bin
    size:>100MB *.data
}
```

## Documentación

- [Especificación de formato](formato/formato_ignore.md)
- [Guía de integración](integracion/integracion_plugin.md)
- [Documentación CLI](cli/referencia_comandos.md)

## Instalación

### Usando Cargo (Recomendado)

```bash
cargo install ignore
```

### Desde el código fuente

```bash
git clone https://github.com/yourusername/dotignore.git
cd dotignore
cargo build --release
```

## Uso

```bash
# Convertir un .gitignore a .ignore
ignore -i .gitignore -o .ignore

# Convertir un .ignore a .gitignore
ignore -c -f git -i .ignore -o .gitignore

# Crear un nuevo archivo .ignore
ignore -n -o .ignore

# Validar un archivo .ignore
ignore -v -i .ignore
```

## Uso de la API

```rust
use ignore::{DotIgnore, ConversionResult};

// Cargar un archivo .ignore
let ignore = DotIgnore::load_from_file(".ignore").unwrap();

// Comprobar si un archivo está ignorado
if ignore.is_ignored("logs/debug.log") {
    println!("Este archivo está ignorado");
}

// Convertir de .gitignore a .ignore
let result = DotIgnore::convert_file(".gitignore", Some(".ignore")).unwrap();
```

## Preservación de Directorios Vacíos

Una característica clave de DotIgnore es el soporte para directorios vacíos sin archivos de marcador como `.gitkeep`. Use el prefijo `&`:

```
# En .ignore
&carpeta-vacia/
```

Esto le indica a tu VCS que preserve el directorio incluso cuando está vacío.

## Licencia

MIT 

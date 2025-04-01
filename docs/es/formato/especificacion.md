# Especificación del formato .DotIgnore

## Introducción

El formato `.DotIgnore` es una mejora sobre los formatos tradicionales de archivos de ignorar (como `.gitignore` o `.svnignore`), que ofrece una estructura semántica más clara, capacidades avanzadas y organización por grupos.

## Estructura básica

Un archivo `.DotIgnore` está estructurado en grupos de patrones, cada uno con un nombre descriptivo y encerrado entre llaves:

```
[nombre_del_grupo] {
    # Comentarios explicativos
    patrón1
    patrón2
    ...
}
```

### Grupos

Los grupos permiten organizar las reglas por categorías lógicas, facilitando la comprensión y mantenimiento. Cada grupo funciona de manera independiente, lo que significa que los patrones en un grupo no afectan a los patrones en otros grupos.

Ejemplos de grupos comunes:

```
[archivos_sistema] {
    # Archivos específicos del sistema operativo
    .DS_Store
    Thumbs.db
}

[compilacion] {
    # Archivos generados en la compilación
    *.o
    *.obj
    bin/
    build/
}
```

## Sintaxis de patrones

### Patrones básicos

La sintaxis de patrones básicos es similar a la de `.gitignore`:

- Líneas en blanco o que comiencen con `#` son ignoradas (comentarios)
- Patrones estándar de glob: `*`, `?`, `[abc]`, etc.
- Los patrones que terminan en `/` solo coinciden con directorios
- Los patrones que comienzan con `!` son negaciones (excluir de ser ignorado)

### Preservación de directorios

Para preservar un directorio vacío (que normalmente sería ignorado), se utiliza el prefijo `&`:

```
[directorios] {
    # Mantener este directorio vacío en el control de versiones
    &test/emptyfolder/    # El prefijo & preserva la estructura del directorio pero ignora su contenido
}
```

### Filtrado por tamaño

Una característica avanzada exclusiva del formato `.DotIgnore` es la capacidad de filtrar archivos por tamaño, lo que permite ignorar archivos basados no solo en su nombre sino también en su tamaño.

Sintaxis:

```
size:<tamaño patrón
size:>tamaño patrón
```

Donde:
- `<` significa "menor que"
- `>` significa "mayor que"
- `tamaño` puede expresarse en bytes, KB, MB o GB

Ejemplos:

```
[archivos_temporales_pequeños] {
    # Ignorar archivos temporales menores a 1MB
    size:<1MB *.tmp
    size:<1MB *.cache
}

[archivos_grandes] {
    # Ignorar archivos binarios mayores a 100MB
    size:>100MB *.bin
    size:>100MB *.iso
    size:>100MB *.dump
}
```

Esto permite configuraciones muy específicas, como ignorar solo los archivos de registro grandes pero mantener los pequeños, o ignorar archivos temporales pequeños pero realizar un seguimiento de los más significativos.

## Organización jerárquica

El formato `.DotIgnore` admite una organización jerárquica mediante el uso de dos puntos para indicar subgrupos:

```
[desarrollo:local] {
    # Configuraciones específicas de desarrollo local
    .env.local
    config.local.json
}

[desarrollo:produccion] {
    # Configuraciones específicas de producción
    .env.prod
    secrets/
}
```

## Ventajas sobre formatos tradicionales

1. **Organización clara**: Los grupos proporcionan una estructura semántica que facilita la comprensión.
2. **Aislamiento de reglas**: Las reglas en diferentes grupos no interactúan entre sí, evitando conflictos.
3. **Capacidades avanzadas**: Funciones como filtrado por tamaño y preservación de directorios.
4. **Documentación mejorada**: La estructura fomenta la documentación clara junto con las reglas.
5. **Conversión automática**: Capacidad para convertir desde y hacia otros formatos.

## Ejemplos completos

### Ejemplo para un proyecto de desarrollo web

```
[sistema] {
    # Archivos del sistema operativo
    .DS_Store
    Thumbs.db
    desktop.ini
}

[editores] {
    # Archivos específicos de editores
    .vscode/
    .idea/
    *.sublime-*
}

[dependencias] {
    # Directorios de dependencias
    node_modules/
    vendor/
    packages/
}

[compilacion] {
    # Archivos de compilación
    dist/
    build/
    *.min.js
    *.min.css
}

[local] {
    # Archivos de configuración local
    .env
    .env.local
    config.local.js
}

[logs] {
    # Archivos de registro
    *.log
    logs/
    # Solo ignorar archivos de registro grandes
    size:>10MB debug.log
}

[temporal] {
    # Archivos temporales
    *.tmp
    *.bak
    *.swp
    # Ignorar archivos temporales pequeños
    size:<1MB *.temp
}

[cache] {
    # Directorios y archivos de caché
    .cache/
    .sass-cache/
    # Preservar directorio de caché pero ignorar su contenido
    &.cache/preserved/
}
```

## Conclusión

El formato `.DotIgnore` representa una evolución significativa sobre los formatos tradicionales de archivos de ignorar, ofreciendo mayor claridad, flexibilidad y capacidades avanzadas como el filtrado por tamaño. Su estructura organizada facilita el mantenimiento y comprensión de las reglas de exclusión en proyectos de cualquier tamaño. 

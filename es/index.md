---
layout: default
title: dotIgnore
permalink: /es/
---

# dotIgnore

> Un formato unificado para archivos de ignorar en todos los sistemas de control de versiones

## Resumen

dotIgnore es un formato estandarizado para definir qué archivos deben ser ignorados en sistemas de control de versiones, compatible con Git, SVN y otros. Ofrece un enfoque más organizado y semántico que los formatos tradicionales, con agrupación, organización jerárquica y características avanzadas.

## Características Clave

- **Reglas de grupo aisladas** - Los patrones dentro de un grupo solo se aplican dentro de su contexto y no afectan a otros grupos, proporcionando mejor organización y previniendo conflictos
- **Validación de sintaxis** - Herramientas de validación integradas que aseguran que tus patrones de ignorar sean correctos antes de aplicarlos
- **Compatibilidad multiplataforma** - Conversión simple entre diferentes formatos de ignorar de distintos VCS
- **Preservación de directorios vacíos** - Seguimiento de directorios vacíos sin archivos de marcador
- **Filtrado basado en tamaño** - Ignorar archivos según su tamaño, no solo por nombres o patrones
- **Estructura semántica clara** - Mejor legibilidad y mantenibilidad con agrupación explícita

## Descripción del Formato

El formato de archivo `.ignore` utiliza una estructura clara y semántica:

```
# Este es un archivo .ignore de ejemplo
# Los patrones fuera de grupos son globales

*.tmp
*.cache

[system] {
    # Archivos del sistema
    .DS_Store
    Thumbs.db
    desktop.ini
}

[build] {
    # Artefactos de compilación
    build/
    dist/
    *.o
    *.obj
}

[logs:app] {
    # Logs de aplicación
    logs/*.log
    !logs/important.log
}

[size:large] {
    # Archivos grandes
    size:>50MB *.bin
    size:>100MB *.data
}

# Preservación de directorios vacíos
&carpeta-vacia/
```

Para más información, consulta el [Convertidor](/docs/es/) o visita el [repositorio en GitHub](https://github.com/sav-vcs/dotignore). 
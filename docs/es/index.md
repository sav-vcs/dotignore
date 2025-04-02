---
layout: default
title: Convertidor de dotIgnore
permalink: /docs/es/
---

# Convertidor de dotIgnore

El convertidor de dotIgnore te permite convertir fácilmente entre diferentes formatos de archivos de ignorar.

## Conversión Entre Formatos

Puedes convertir entre los formatos `.gitignore`, `.svnignore` y `.ignore` usando nuestra herramienta de conversión simple.

### Interfaz de Línea de Comandos

```bash
# Convertir un .gitignore a .ignore
vcsconvert -i .gitignore -o .ignore

# Convertir un .ignore a .gitignore
vcsconvert -i .ignore -o .gitignore

# Convertir un .svnignore a .ignore
vcsconvert -i .svnignore -o .ignore

# Convertir un .ignore a .svnignore
vcsconvert -i .ignore -o .svnignore
```

### Opciones Adicionales

```bash
# Mostrar ayuda y todas las opciones disponibles
vcsconvert --help
```

## Instalación

El convertidor está disponible para diferentes sistemas operativos:

- Windows: `bin/win/vcsconvert.exe`
- macOS: `bin/macos/vcsconvert`
- Linux: `bin/linux/vcsconvert`

## Integración con SAV

dotIgnore está diseñado para ser usado con SAV (Semantic Artifact Versioning). Cuando se usa SAV, dotIgnore se instala e integra automáticamente.

## Opciones de Idioma

- [Convertidor en Inglés](/docs/en/)
- [Convertidor en Español](/docs/es/)

## Contribuir

Si quieres contribuir al proyecto, por favor visita nuestro [repositorio GitHub](https://github.com/sav-vcs/dotignore). 

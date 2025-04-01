# Documentación de DotIgnore

Bienvenido a la documentación de DotIgnore. Esta guía te ayudará a entender cómo usar la herramienta y el formato DotIgnore.

## Índice de Contenidos

- [Formato .DotIgnore](formato/DotIgnore-format.md)
- [Integración con VCS](integracion/index.md)
- [Uso del CLI](cli/index.md)
- [Preguntas Frecuentes](faq.md)

## ¿Qué es DotIgnore?

DotIgnore es una solución estándar para archivos de ignorar en sistemas de control de versiones que mejora los formatos como `.gitignore` y `.svnignore`. El proyecto proporciona tanto un formato estandarizado para archivos de ignorar como una implementación modular que puede integrarse en cualquier sistema de control de versiones.

## Características Principales

- **Formato Estandarizado**: Un formato mejorado para archivos de ignorar con agrupación semántica de patrones
- **Grupos basados en Comentarios**: Crea automáticamente grupos basados en los comentarios del archivo original
- **Soporte Multiplataforma**: Funciona en Windows, macOS y Linux
- **Opciones de CLI y Plugin**: Úsalo como herramienta independiente o intégralo en tu VCS

## Primeros Pasos

Para comenzar rápidamente con DotIgnore, puedes:

1. **Convertir un archivo de ignorar existente**:
   ```bash
   dotignore convert-ignore -f git -s .gitignore -d .DotIgnore
   ```

2. **Integrar con tu VCS como plugin** (para desarrolladores):
   ```rust
   let dotignore = DotIgnore::load_from_file(".DotIgnore")?;
   let should_ignore = dotignore.is_ignored("ruta/al/archivo.txt");
   ```

Explora las secciones de documentación para obtener información más detallada sobre el uso de DotIgnore en tus proyectos. 

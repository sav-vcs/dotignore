# Formato del archivo .ignore

Este documento describe el formato del archivo `.ignore` utilizado en el sistema dotIgnore para definir patrones de archivos a ignorar.

## Introducción

El archivo `.ignore` es un archivo de configuración que le indica al sistema qué archivos y directorios debe ignorar. A diferencia de los formatos tradicionales como `.gitignore`, el formato `.ignore` ofrece una estructura más organizada y características adicionales.

Para más información y documentación actualizada, visite [https://dotignore.dev](https://dotignore.dev).

## Estructura

Un archivo `.ignore` está estructurado en grupos de patrones, lo que permite organizar los patrones por categoría y propósito. Los grupos se crean automáticamente a partir de los comentarios en el archivo y cada grupo está delimitado por corchetes y llaves.

Formato general:

```
# Comentario que define el propósito del grupo
[nombre_del_grupo] {
    patrón1
    patrón2
    ...
}
```

El nombre del grupo se deriva automáticamente del texto del comentario que lo precede, convirtiendo ese texto a un formato "slug" (removiendo caracteres especiales y reemplazando espacios con guiones bajos).

## Grupos Basados en Comentarios

Cuando escribes un comentario en tu archivo `.ignore`, ese comentario se convierte en el nombre del grupo para todos los patrones que le siguen hasta encontrar otro comentario o una línea en blanco.

Por ejemplo:

```
# Archivos temporales
*.tmp
*.log
*.bak

# Archivos del sistema
Thumbs.db
.DS_Store
```

Se procesará como:

```
[archivos_temporales] {
    *.tmp
    *.log
    *.bak
}

[archivos_del_sistema] {
    Thumbs.db
    .DS_Store
}
```

Si no hay un comentario previo, los patrones se agrupan en un grupo llamado `default`.

## Sintaxis de Patrones

Los patrones en `.ignore` siguen estas reglas:

- Líneas en blanco o que comienzan con `#` son comentarios
- Los patrones que comienzan con `!` son negados (incluir específicamente)
- Los patrones que comienzan con `&` mantienen la estructura del directorio pero ignoran el contenido
- Los patrones que terminan con `/` solo coinciden con directorios
- `*` coincide con cualquier carácter excepto separadores de ruta
- `**` coincide con cualquier carácter incluyendo separadores de ruta (recursivo)
- Los patrones sin `/` se aplican a cualquier parte de la ruta
- Los patrones que comienzan con `/` se aplican desde la raíz del repositorio

## Ejemplo Completo

```
# Archivos del sistema
[archivos_del_sistema] {
    # Windows
    Thumbs.db
    desktop.ini
    
    # macOS
    .DS_Store
    ._*
}

# Archivos de construcción
[archivos_de_construccion] {
    # Binarios compilados
    *.o
    *.obj
    *.exe
    
    # Directorios de salida
    /build/
    /dist/
    
    # Pero mantener archivos específicos
    !build/importante.txt
}

# Dependencias y paquetes
[dependencias] {
    # Gestores de paquetes comunes
    /node_modules/
    /vendor/
    /packages/
}
```

## Ejemplos de Patrones Avanzados

| Patrón | Descripción |
|--------|-------------|
| `*.log` | Ignora todos los archivos .log en cualquier directorio |
| `!debug.log` | Incluye archivo debug.log aunque otros .log estén ignorados |
| `/logs/` | Ignora directorio logs en la raíz del proyecto |
| `doc/*.txt` | Ignora .txt solo en directorio doc/ |
| `doc/**/*.txt` | Ignora .txt en doc/ y todos sus subdirectorios |
| `&cache/` | Mantiene directorio cache/ pero ignora su contenido |
| `**/build/` | Ignora directorio build/ en cualquier nivel |
| `**/logs/*.log` | Ignora archivos .log en cualquier directorio logs/ |
| `/src/**/*.bak` | Ignora archivos .bak en /src/ y sus subdirectorios |

## Capacidades Avanzadas

El formato `.ignore` incluye varias capacidades avanzadas que no están disponibles en otros formatos:

### 1. Jerarquía de Herencia

Los patrones se evalúan en orden de especificidad y se aplica el último que coincide. Las reglas de negación (`!`) pueden anular patrones anteriores, permitiendo excepciones específicas.

### 2. Patrones de Preservación de Directorios

El operador `&` permite mantener un directorio en el sistema de archivos, pero ignorar todos sus contenidos. Esto es útil para:

- Mantener la estructura de directorios pero ignorar archivos temporales
- Preservar directorios vacíos que son necesarios para la compilación
- Asegurar que la estructura de carpetas se mantenga constante

Ejemplo:
```
&logs/debug/
```
Esto mantendrá el directorio `logs/debug/` pero ignorará todo su contenido.

### 3. Organización Semántica

El uso de comentarios para definir grupos proporciona una organización semántica natural. Los patrones relacionados se mantienen juntos, lo que mejora la legibilidad y el mantenimiento del archivo.

## Conversión Automática

dotIgnore ofrece herramientas para convertir automáticamente archivos `.gitignore` y `.svnignore` al formato `.ignore`. La conversión:

1. Lee el archivo original línea por línea
2. Crea grupos basados en los comentarios encontrados
3. Agrupa los patrones bajo el grupo correspondiente
4. Genera un archivo `.ignore` bien estructurado

Para convertir un archivo:

```bash
dotignore convert-ignore -s .gitignore
```

## Ventajas sobre otros formatos

El formato `.ignore` ofrece varias ventajas:

1. **Organización semántica**: Agrupa patrones según el propósito indicado en los comentarios
2. **Conservación de la intención**: Mantiene la estructura original con los mismos comentarios
3. **Mayor legibilidad**: La estructura de grupos facilita la lectura y mantenimiento
4. **Conversión intuitiva**: El proceso de conversión respeta la estructura original
5. **Flexibilidad**: No impone categorías predefinidas, se adapta a las necesidades específicas

## Especificaciones Técnicas

- El archivo debe estar codificado en UTF-8
- El nombre de los grupos se genera a partir de los comentarios, eliminando caracteres especiales
- Una línea en blanco o un nuevo comentario finaliza el grupo actual
- Se recomienda usar comentarios descriptivos que expliquen claramente el propósito de los patrones

---

Para documentación detallada y actualizaciones, visite [https://dotignore.dev](https://dotignore.dev)  

Desarrollado con el apoyo de [SAV Project](https://www.sav-project.com) 
# DotIgnore Format Specification

This document describes the specification of the `.DotIgnore` standard, an improved and more intuitive format for specifying files that should be ignored by version control systems.

## Introduction

The `.DotIgnore` format is an improvement over traditional ignore file formats (like `.gitignore` or `.svnignore`), offering clearer semantic structure, advanced capabilities, and group organization.

## Basic Structure

A `.DotIgnore` file is structured into pattern groups, each with a descriptive name and enclosed in curly braces:

```
[group_name] {
    # Explanatory comments
    pattern1
    pattern2
    ...
}
```

### Groups

Groups allow organizing rules by logical categories, making them easier to understand and maintain. Each group functions independently, meaning patterns in one group don't affect patterns in other groups.

Examples of common groups:

```
[system_files] {
    # OS-specific files
    .DS_Store
    Thumbs.db
}

[build] {
    # Files generated during build process
    *.o
    *.obj
    bin/
    build/
}
```

## Pattern Syntax

### Basic Patterns

The basic pattern syntax is similar to `.gitignore`:

- Blank lines or lines starting with `#` are ignored (comments)
- Standard glob patterns: `*`, `?`, `[abc]`, etc.
- Patterns ending with `/` only match directories
- Patterns starting with `!` are negations (exclude from being ignored)

### Directory Preservation

To preserve an empty directory (which would normally be ignored), use the `&` prefix:

```
[directories] {
    # Keep this empty directory in version control
    &test/emptyfolder/    # The & prefix preserves directory structure but ignores contents
}
```

### Size-based Filtering

An advanced feature exclusive to the `.DotIgnore` format is the ability to filter files by size, allowing you to ignore files based not only on their name but also on their size.

Syntax:

```
size:<size pattern
size:>size pattern
```

Where:
- `<` means "less than"
- `>` means "greater than"
- `size` can be expressed in bytes, KB, MB, or GB

Examples:

```
[small_temp_files] {
    # Ignore temporary files smaller than 1MB
    size:<1MB *.tmp
    size:<1MB *.cache
}

[large_files] {
    # Ignore binary files larger than 100MB
    size:>100MB *.bin
    size:>100MB *.iso
    size:>100MB *.dump
}
```

This allows for very specific configurations, such as ignoring only large log files but keeping small ones, or ignoring small temporary files but tracking more significant ones.

## Hierarchical Organization

The `.DotIgnore` format supports hierarchical organization through the use of colons to indicate subgroups:

```
[development:local] {
    # Local development specific configurations
    .env.local
    config.local.json
}

[development:production] {
    # Production specific configurations
    .env.prod
    secrets/
}
```

## Advantages Over Traditional Formats

1. **Clear Organization**: Groups provide a semantic structure that makes rules easier to understand.
2. **Rule Isolation**: Rules in different groups don't interact with each other, avoiding conflicts.
3. **Advanced Capabilities**: Features like size filtering and directory preservation.
4. **Improved Documentation**: The structure encourages clear documentation alongside rules.
5. **Automatic Conversion**: Ability to convert to and from other formats.

## Complete Examples

### Example for a Web Development Project

```
[system] {
    # Operating system files
    .DS_Store
    Thumbs.db
    desktop.ini
}

[editors] {
    # Editor specific files
    .vscode/
    .idea/
    *.sublime-*
}

[dependencies] {
    # Dependency directories
    node_modules/
    vendor/
    packages/
}

[build] {
    # Build files
    dist/
    build/
    *.min.js
    *.min.css
}

[local] {
    # Local configuration files
    .env
    .env.local
    config.local.js
}

[logs] {
    # Log files
    *.log
    logs/
    # Only ignore large log files
    size:>10MB debug.log
}

[temp] {
    # Temporary files
    *.tmp
    *.bak
    *.swp
    # Ignore small temporary files
    size:<1MB *.temp
}

[cache] {
    # Cache directories and files
    .cache/
    .sass-cache/
    # Preserve cache directory but ignore its contents
    &.cache/preserved/
}
```

## Conclusion

The `.DotIgnore` format represents a significant evolution over traditional ignore file formats, offering greater clarity, flexibility, and advanced capabilities like size filtering. Its organized structure makes maintaining and understanding exclusion rules easier in projects of any size. 

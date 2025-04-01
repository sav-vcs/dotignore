# .ignore File Format

This document describes the `.ignore` file format used in the dotIgnore system to define file patterns to ignore.

## Introduction

The `.ignore` file is a configuration file that tells the system which files and directories to ignore. Unlike traditional formats like `.gitignore`, the `.ignore` format offers a more organized structure and additional features.

For more information and up-to-date documentation, visit [https://dotignore.dev](https://dotignore.dev).

## Structure

A `.ignore` file is structured in pattern groups, allowing patterns to be organized by category and purpose. Groups are automatically created from comments in the file, and each group is delimited by brackets and braces.

General format:

```
# Comment that defines the group's purpose
[group_name] {
    pattern1
    pattern2
    ...
}
```

The group name is automatically derived from the comment text that precedes it, converting that text to a "slug" format (removing special characters and replacing spaces with underscores).

## Comment-Based Groups

When you write a comment in your `.ignore` file, that comment becomes the group name for all patterns that follow until another comment or blank line is found.

For example:

```
# Temporary files
*.tmp
*.log
*.bak

# System files
Thumbs.db
.DS_Store
```

Will be processed as:

```
[temporary_files] {
    *.tmp
    *.log
    *.bak
}

[system_files] {
    Thumbs.db
    .DS_Store
}
```

If there is no preceding comment, patterns are grouped under a `default` group.

## Pattern Syntax

Patterns in `.ignore` follow these rules:

- Blank lines or lines starting with `#` are comments
- Patterns starting with `!` are negated (explicitly include)
- Patterns starting with `&` maintain directory structure but ignore contents
- Patterns ending with `/` only match directories
- `*` matches any character except path separators
- `**` matches any character including path separators (recursive)
- Patterns without `/` apply to any part of the path
- Patterns starting with `/` apply from the repository root

## Complete Example

```
# System files
[system_files] {
    # Windows
    Thumbs.db
    desktop.ini
    
    # macOS
    .DS_Store
    ._*
}

# Build files
[build_files] {
    # Compiled binaries
    *.o
    *.obj
    *.exe
    
    # Output directories
    /build/
    /dist/
    
    # But keep specific files
    !build/important.txt
}

# Dependencies and packages
[dependencies] {
    # Common package managers
    /node_modules/
    /vendor/
    /packages/
}
```

## Advanced Pattern Examples

| Pattern | Description |
|---------|-------------|
| `*.log` | Ignore all .log files in any directory |
| `!debug.log` | Include debug.log even if other .log files are ignored |
| `/logs/` | Ignore logs directory in project root |
| `doc/*.txt` | Ignore .txt only in doc/ directory |
| `doc/**/*.txt` | Ignore .txt in doc/ and all subdirectories |
| `&cache/` | Keep cache/ directory but ignore its contents |
| `**/build/` | Ignore build/ directory at any level |
| `**/logs/*.log` | Ignore .log files in any logs/ directory |
| `/src/**/*.bak` | Ignore .bak files in /src/ and subdirectories |

## Advanced Features

The `.ignore` format includes several advanced features not available in other formats:

### 1. Inheritance Hierarchy

Patterns are evaluated in order of specificity, and the last matching pattern is applied. Negation rules (`!`) can override previous patterns, allowing specific exceptions.

### 2. Directory Preservation Patterns

The `&` operator allows keeping a directory in the filesystem while ignoring all its contents. This is useful for:

- Maintaining directory structure but ignoring temporary files
- Preserving empty directories needed for compilation
- Ensuring folder structure remains constant

Example:
```
&logs/debug/
```
This will keep the `logs/debug/` directory but ignore all its contents.

### 3. Semantic Organization

Using comments to define groups provides natural semantic organization. Related patterns are kept together, improving file readability and maintainability.

## Automatic Conversion

DotIgnore provides tools to automatically convert `.gitignore` and `.svnignore` files to the `.DotIgnore` format. The conversion:

1. Reads the original file line by line
2. Creates groups based on found comments
3. Groups patterns under their corresponding group
4. Generates a well-structured `.DotIgnore` file

To convert a file:

```bash
dotignore convert-ignore -s .gitignore
```

## Advantages over other formats

The `.DotIgnore` format offers several advantages:

1. **Semantic organization**: Groups patterns according to purpose indicated in comments
2. **Intent preservation**: Maintains original structure with same comments
3. **Better readability**: Group structure facilitates reading and maintenance
4. **Intuitive conversion**: Conversion process respects original structure
5. **Flexibility**: Doesn't impose predefined categories, adapts to specific needs

## Technical Specifications

- File must be UTF-8 encoded
- Group names are generated from comments, removing special characters
- A blank line or new comment ends the current group
- Descriptive comments are recommended to clearly explain pattern purposes

---

For detailed documentation and updates, visit [https://dotignore.dev](https://dotignore.dev)

Developed with the support of [SAV Project](https://www.sav-project.com) 
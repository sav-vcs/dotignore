# .ignore Format Converter

This document describes how to use the format converter that allows migrating from `.gitignore` and `.svnignore` files to the new `.ignore` standard format of the dotIgnore system.

For complete and up-to-date documentation, visit [https://dotignore.dev/docs/converter](https://dotignore.dev/docs/converter).

## Features

- **Format Conversion**: Transforms Git and SVN rules to .ignore format
- **Comment-based Organization**: Creates groups based on comments in the original file
- **Structure Preservation**: Maintains original comments and organization

## Usage

### Convert an Individual File

```bash
dotignore convert-ignore .gitignore
```

This will create a `.ignore` file in the same directory, keeping the original.

### Specify Destination Path

```bash
dotignore convert-ignore .gitignore /path/to/destination/.ignore
```

### View Help

```bash
dotignore convert-ignore --help
```

## Conversion Format

The converter applies the following transformations:

### From .gitignore

1. Creates groups based on comments found in the file
2. If no comments are found, groups patterns in the `[default]` section
3. Keeps all original comments

### From .svnignore

1. Creates groups based on comments found in the file
2. If no comments are found, groups patterns in the `[default]` section
3. Keeps all original comments

## Examples

### Original .gitignore File:

```
# Binary files
*.exe
*.dll

# System
.DS_Store
Thumbs.db
```

### Resulting .ignore File:

```
# .ignore file converted from .gitignore
# Conversion date: 2023-08-15 10:30:45

[binary_files] {
    *.exe
    *.dll
} # end binary_files

[system] {
    .DS_Store
    Thumbs.db
} # end system
```

## Building from Source

To build the converter from source:

1. Make sure you have .NET SDK 7.0 or higher installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/sav-project/dotignore-converter.git
   ```
3. Build the project:
   ```bash
   cd dotignore-converter
   dotnet build -c Release
   ```

## Troubleshooting

### Converter Doesn't Recognize the Format Correctly

Make sure the files have the correct names (`.gitignore` or `.svnignore`). If you have a file with Git format but with another name, rename it temporarily for conversion.

## License

The format converter is distributed under the MIT license.

---

Developed with the support of [SAV Project](https://www.sav-project.com) 
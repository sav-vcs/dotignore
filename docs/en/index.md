---
layout: default
title: dotIgnore Converter
permalink: /docs/en/
---

# dotIgnore Converter

The dotIgnore converter allows you to easily convert between different ignore file formats.

## Converting Between Formats

You can convert between `.gitignore`, `.svnignore`, and `.ignore` formats using our simple conversion tool.

### Command Line Interface

```bash
# Convert a .gitignore to .ignore
vcsconvert -i .gitignore -o .ignore

# Convert a .ignore to .gitignore
vcsconvert -i .ignore -o .gitignore

# Convert a .svnignore to .ignore
vcsconvert -i .svnignore -o .ignore

# Convert a .ignore to .svnignore
vcsconvert -i .ignore -o .svnignore
```

### Additional Options

```bash
# Show help and all available options
vcsconvert --help
```

## Installation

The converter is available for different operating systems:

- Windows: `bin/win/vcsconvert.exe`
- macOS: `bin/macos/vcsconvert`
- Linux: `bin/linux/vcsconvert`

## Integration with SAV

dotIgnore is designed to be used with SAV (Semantic Artifact Versioning). When using SAV, dotIgnore is automatically installed and integrated.

## Language Options

- [English Converter](/docs/en/)
- [Spanish Converter](/docs/es/)

## Contributing

If you want to contribute to the project, please visit our [GitHub repository](https://github.com/sav-vcs/dotignore).

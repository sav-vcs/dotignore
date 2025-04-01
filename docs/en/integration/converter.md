# DotIgnore Format Converter

This document describes how to use the format converter that allows migrating from `.gitignore` and `.svnignore` files to the new `.DotIgnore` standard format.

## Features

- **Smart Conversion**: Automatically transforms Git and SVN rules to .DotIgnore format.
- **Structured Organization**: Automatically classifies patterns into semantic groups.
- **Platform Identification**: Detects OS-specific patterns and organizes them into appropriate groups.
- **Batch Conversion**: Allows converting all files in a repository or directory recursively.
- **Self-installing**: Installs as a command-line tool on any operating system.
- **Cross-platform**: Available for Windows, macOS, and Linux.

## Installation

### Quick Method

Run the following command to install the converter:

```bash
DotIgnore-converter --install
```

This will install the tool in an appropriate location according to your operating system and make it available globally.

### Manual Installation

1. Download the `DotIgnore-converter-{version}-<platform>.zip` or `.tar.gz` file from the downloads section.
2. Extract the contents to any directory.
3. Follow the specific instructions for your operating system:

   **Windows:**
   - Add the directory to the system PATH, or
   - Create a shortcut in an accessible location.

   **macOS/Linux:**
   - Create a symbolic link: `ln -s /path/to/DotIgnore-converter /usr/local/bin/`

## Usage

### Convert an Individual File

```bash
DotIgnore-converter .gitignore
```

This will create a `.DotIgnore` file in the same directory, keeping the original.

### Specify Destination Path

```bash
DotIgnore-converter .gitignore /path/to/destination/.DotIgnore
```

### Convert an Entire Directory (Recursively)

```bash
DotIgnore-converter /path/to/repository
```

This will search for and convert all `.gitignore` and `.svnignore` files in the directory and subdirectories.

### View Help

```bash
DotIgnore-converter --help
```

## Conversion Format

The converter applies the following transformations:

### From .gitignore

1. Groups general patterns in the `[standard_patterns]` section
2. Identifies and separates patterns specific to:
   - Windows: `[windows]`
   - macOS: `[macos]`
   - Linux: `[linux]`
3. Creates an additional group for IDEs/editors: `[editors]`
4. Keeps all original comments

### From .svnignore

1. Groups patterns in the `[svn_patterns]` section
2. Converts the space-separated format to individual lines
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

### Resulting .DotIgnore File:

```
# .DotIgnore file converted from .gitignore
# Conversion date: 2023-08-15 10:30:45

[standard_patterns] {
    # Binary files
    *.exe
    *.dll
}

[macos] {
    .DS_Store
}

[windows] {
    Thumbs.db
}

[editors] {
    # Files specific to editors/IDEs
    # Comment or uncomment as needed
    #.idea/
    #.vscode/
    #*.sublime-*
    #*.swp
}
```

## Tool Integration

### Integration with Git (hooks)

You can set up a post-merge hook to automatically convert .gitignore files:

```bash
#!/bin/sh
# .git/hooks/post-merge

DotIgnore-converter .gitignore
```

### Integration with CI/CD Pipelines

Add the conversion to your CI/CD workflows:

```yaml
# GitHub Actions
- name: Convert ignore files
  run: |
    curl -sSL https://DotIgnore.com/download/DotIgnore-converter | bash
    DotIgnore-converter $GITHUB_WORKSPACE
```

## Troubleshooting

### Converter Doesn't Recognize the Format Correctly

Make sure the files have the correct names (`.gitignore` or `.svnignore`). If you have a file with Git format but with another name, rename it temporarily for conversion.

### Permission Error on Linux/macOS

If there are permission issues when trying to install or run:

```bash
chmod +x /path/to/DotIgnore-converter
sudo DotIgnore-converter --install
``` 

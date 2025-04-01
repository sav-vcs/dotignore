# Cross-Platform Validator for .DotIgnore Standard

This document describes how to use the cross-platform validator for `.DotIgnore` files that comply with the standard.

## Features

- **Cross-platform**: Standalone executables for Windows, macOS, and Linux.
- **Complete validation**: Verifies the structure, syntax, and coherence of your `.DotIgnore` files.
- **Clear messages**: Provides specific error and warning messages with correction suggestions.
- **Batch mode**: Allows validating all `.DotIgnore` files in a directory and its subdirectories.
- **Easy integration**: Can be integrated into CI/CD workflows and automation scripts.

## Installation

### Windows

1. Download the `DotIgnore-validator-{version}-win-x64.zip` file from the downloads section.
2. Extract the contents to any directory of your choice.
3. Optionally, add the directory path to the PATH to run the validator from any location.

### macOS

1. Download the `DotIgnore-validator-{version}-osx-x64.tar.gz` file from the downloads section.
2. Extract the contents:
   ```bash
   tar -xzf DotIgnore-validator-{version}-osx-x64.tar.gz -C /destination/path
   ```
3. Give execution permissions:
   ```bash
   chmod +x /destination/path/DotIgnore-validator
   ```
4. Optionally, create a symbolic link:
   ```bash
   ln -s /destination/path/DotIgnore-validator /usr/local/bin/DotIgnore-validator
   ```

### Linux

1. Download the `DotIgnore-validator-{version}-linux-x64.tar.gz` or `DotIgnore-validator-{version}-linux-arm64.tar.gz` file according to your architecture from the downloads section.
2. Extract the contents:
   ```bash
   tar -xzf DotIgnore-validator-{version}-linux-x64.tar.gz -C /destination/path
   ```
3. Give execution permissions:
   ```bash
   chmod +x /destination/path/DotIgnore-validator
   ```
4. Optionally, create a symbolic link:
   ```bash
   sudo ln -s /destination/path/DotIgnore-validator /usr/local/bin/DotIgnore-validator
   ```

## Usage

### Validate a Specific File

```bash
DotIgnore-validator /path/to/file/.DotIgnore
```

### Validate All Files in a Directory

```bash
DotIgnore-validator --batch /path/to/directory
```

### View Current Version

```bash
DotIgnore-validator --version
```

### Display Help

```bash
DotIgnore-validator --help
```

## Exit Codes

- **0**: Validation completed successfully without errors (there may be warnings).
- **1**: Errors were encountered during validation.

## Examples

### Validate a Local File

```bash
DotIgnore-validator .DotIgnore
```

### Validate a File at a Specific Path

```bash
DotIgnore-validator C:\projects\my-repo\.DotIgnore  # Windows
DotIgnore-validator /home/user/projects/my-repo/.DotIgnore  # Unix
```

### Validate All .DotIgnore Files in a Directory

```bash
DotIgnore-validator --batch C:\projects  # Windows
DotIgnore-validator --batch /home/user/projects  # Unix
```

## Integration with Development Environments (IDEs)

### Visual Studio Code

Create a task file in `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Validate .DotIgnore",
      "type": "shell",
      "command": "DotIgnore-validator",
      "args": ["${workspaceFolder}/.DotIgnore"],
      "presentation": {
        "reveal": "always",
        "panel": "new"
      },
      "group": {
        "kind": "build",
        "isDefault": true
      }
    }
  ]
}
```

### Integration with Git Hooks

You can create a pre-commit hook to validate the `.DotIgnore` file before committing changes:

```bash
#!/bin/sh
# .git/hooks/pre-commit

DotIgnore-validator .DotIgnore
if [ $? -ne 0 ]; then
  echo "Error: The .DotIgnore file has errors. Please fix them before committing."
  exit 1
fi
```

## Troubleshooting

### The Executable Doesn't Work on macOS

If macOS prevents you from running the validator because it comes from an "unidentified developer," you can allow its execution in the following ways:

1. From Finder, right-click on the executable and select "Open".
2. Confirm that you want to open it in the dialog that appears.

Or, from the terminal:

```bash
xattr -d com.apple.quarantine /path/to/DotIgnore-validator
```

### The Executable Doesn't Work on Linux

Make sure you have the correct execution permissions:

```bash
chmod +x /path/to/DotIgnore-validator
``` 

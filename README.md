# dotIgnore

> A unified format for ignore files across all version control systems

## Overview

dotIgnore is a standardized format to define which files should be ignored in version control systems, compatible with Git, SVN, and others. It offers a more organized and semantic approach than traditional formats, with grouping, hierarchical organization, and advanced features.

[Read this in Spanish](docs/es/README.md)

## Key Features

- **Organized groups** to categorize ignore patterns
- **Cross-platform compatibility** with simple conversion between formats
- **Empty directory preservation** without placeholder files
- **Size-based filtering** to ignore files based on their size
- **Clear syntax** with better readability and maintainability

## Usage

dotIgnore is designed to be used with SAV (Semantic Artifact Versioning). When using SAV, dotIgnore is automatically installed and integrated.

You can also use the standalone conversion tool:

```bash
vcsconvert -i .gitignore -o .ignore
```

The executable files are available in the `bin` directory and do not require installation.

> **Note:** Currently only Windows binaries are available. Linux and macOS binaries will be added soon.

## Format

# dotIgnore format example

```
# This is an example .ignore file

[system] {
    # System files
    .DS_Store
    Thumbs.db
    desktop.ini
    *.tmp
    *.cache
}

[build] {
    # Build artifacts
    build/
    dist/
    *.o
    *.obj
}

[logs_app] {
    # Application logs
    logs/*.log
    !logs/important.log
}

[size_large] {
    # Large files
    size:>50MB *.bin
    size:>100MB *.data
}

[empty_directories] {
    # Empty directory preservation
    &empty-folder/
}

## Documentation

- [Format Specification](docs/en/format/dotignore-format.md)
- [Integration Guide](docs/en/integration/index.md)
- [CLI Documentation](docs/en/cli/index.md)

### Source

```bash
git clone https://github.com/yourusername/dotignore.git
cd dotignore
cargo build --release
```

## Usage of converter

```bash
# Convert a .gitignore to .ignore
scvconvert -i .gitignore -o .ignore

# Convert a .ignore to .gitignore
scvconvert -c -f git -i .ignore -o .gitignore

# Create a new .ignore file
scvconvert -n -o .ignore

# Validate a .ignore file
scvconvert -v -i .ignore
```

## API Usage

```rust
use ignore::{DotIgnore, ConversionResult};

// Load a .ignore file
let ignore = DotIgnore::load_from_file(".ignore").unwrap();

// Check if a file is ignored
if ignore.is_ignored("logs/debug.log") {
    println!("This file is ignored");
}

// Convert from .gitignore to .ignore
let result = DotIgnore::convert_file(".gitignore", Some(".ignore")).unwrap();
```

## Empty Directory Preservation

One key feature of dotIgnore is supporting empty directories without placeholder files like `.gitkeep`. Use the `&` prefix as shown in the example above.

## License

MIT 

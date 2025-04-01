# DotIgnore

> A unified format for ignore files across all version control systems

## Overview

DotIgnore is a standardized format to define which files should be ignored in version control systems, compatible with Git, SVN, and others. It offers a more organized and semantic approach than traditional formats, with grouping, hierarchical organization, and advanced features.

[Read this in Spanish](docs/es/README.md)

## Key Features

- **Organized groups** to categorize ignore patterns
- **Cross-platform compatibility** with simple conversion between formats
- **Empty directory preservation** without placeholder files
- **Size-based filtering** to ignore files based on their size
- **Clear syntax** with better readability and maintainability

## Quick Start

1. **Install DotIgnore**:

   ```bash
   cargo install ignore
   ```

2. **Convert your existing ignore files**:

   ```bash
   ignore -i .gitignore -o .ignore
   ```

3. **Or create a new DotIgnore file**:

   ```bash
   ignore -n -o .ignore
   ```

## Format

# DotIgnore format example

```
# This is an example .ignore file
# Patterns outside groups are global

*.tmp
*.cache

[system] {
    # System files
    .DS_Store
    Thumbs.db
    desktop.ini
}

[build] {
    # Build artifacts
    build/
    dist/
    *.o
    *.obj
}

[logs:app] {
    # Application logs
    logs/*.log
    !logs/important.log
}

[size:large] {
    # Large files
    size:>50MB *.bin
    size:>100MB *.data
}
```

## Documentation

- [Format Specification](docs/en/format/ignore_format.md)
- [Integration Guide](docs/en/integration/plugin_integration.md)
- [CLI Documentation](docs/en/cli/command_reference.md)

## Installation

### Using Cargo (Recommended)

```bash
cargo install ignore
```

### From Source

```bash
git clone https://github.com/yourusername/dotignore.git
cd dotignore
cargo build --release
```

## Usage

```bash
# Convert a .gitignore to .ignore
ignore -i .gitignore -o .ignore

# Convert a .ignore to .gitignore
ignore -c -f git -i .ignore -o .gitignore

# Create a new .ignore file
ignore -n -o .ignore

# Validate a .ignore file
ignore -v -i .ignore
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

One key feature of DotIgnore is supporting empty directories without placeholder files like `.gitkeep`. Use the `&` prefix:

```
# In .ignore
&empty-folder/
```

This tells your VCS to preserve the directory even when empty.

## License

MIT 

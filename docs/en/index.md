# DotIgnore Documentation

Welcome to the DotIgnore documentation. This guide will help you understand how to use the DotIgnore tool and format.

## Table of Contents

- [.DotIgnore Format](format/DotIgnore-format.md)
- [VCS Integration](integration/index.md)
- [CLI Usage](cli/index.md)
- [FAQ](faq.md)

## What is DotIgnore?

DotIgnore is a standard solution for ignore files in version control systems that improves upon formats like `.gitignore` and `.svnignore`. The project provides both a standardized format for ignore files and a modular implementation that can be integrated into any version control system.

## Key Features

- **Standardized Format**: An improved format for ignore files with semantic grouping of patterns
- **Comment-based Groups**: Automatically creates groups based on comments in the original file
- **Cross-Platform Support**: Works on Windows, macOS, and Linux
- **CLI and Plugin Options**: Use as a standalone tool or integrate into your VCS

## Getting Started

To quickly get started with DotIgnore, you can:

1. **Convert an existing ignore file**:
   ```bash
   dotignore convert-ignore -f git -s .gitignore -d .DotIgnore
   ```

2. **Integrate with your VCS as a plugin** (for developers):
   ```rust
   let dotignore = DotIgnore::load_from_file(".DotIgnore")?;
   let should_ignore = dotignore.is_ignored("path/to/file.txt");
   ```

Explore the documentation sections for more detailed information on using DotIgnore in your projects. 

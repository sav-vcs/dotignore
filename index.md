---
layout: default
title: dotIgnore
permalink: /
---

# dotIgnore

> A unified format for ignore files across all version control systems

## Overview

dotIgnore is a standardized format to define which files should be ignored in version control systems, compatible with Git, SVN, and others. It offers a more organized and semantic approach than traditional formats, with grouping, hierarchical organization, and advanced features.

## Key Features

- **Isolated group rules** - Patterns within groups only apply within their context and don't affect other groups, providing better organization and preventing conflicts
- **Syntax validation** - Built-in validation tools ensure your ignore patterns are correct before they're applied
- **Cross-platform compatibility** - Simple conversion between different VCS ignore formats
- **Empty directory preservation** - Track empty directories without placeholder files
- **Size-based filtering** - Ignore files based on their size, not just names or patterns
- **Clear semantic structure** - Better readability and maintainability with explicit grouping

## Format Description

The `.ignore` file format uses a clear, semantic structure:

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

# Empty directory preservation
&empty-folder/
```

For more information, check the [Converter](/docs/) or visit the [GitHub repository](https://github.com/sav-vcs/dotignore).
